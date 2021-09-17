package main

import (
	"fmt"
	"io"
	"log"
	"net/http"
	"os"
	"strings"
	"time"

	"golang.org/x/net/html"
	tb "gopkg.in/tucnak/telebot.v2"
)

func main() {
	b, err := tb.NewBot(tb.Settings{
		Token: os.Getenv("VCLIPBOT_TELEGRAM_TOKEN"),
		Poller: &tb.LongPoller{
			Timeout: 10 * time.Second,
		},
	})
	if err != nil {
		log.Fatal(err)
	}

	b.Handle(tb.OnText, func(m *tb.Message) {
		if _, err := b.Send(
			m.Sender,
			fmt.Sprintf(
				"%s\n\n%s\n",
				"This bot can help you find and share video clips. It works automatically, no need to add it anywhere. Simply open any of your chats and type `@vclipbot something` in the message field. Then tap on a result to send.",
				"For example, try typing `@vclipbot goat` here.",
			),
			tb.ModeMarkdown,
		); err != nil {
			log.Printf("error: %s\n", err)
		}
	})

	b.Handle(tb.OnQuery, func(q *tb.Query) {
		handleErr := func(err error) {
			log.Printf("error: %s\n", err)
			return
		}

		var resp *http.Response
		if q.Text == "" {
			resp, err = http.Get("https://getyarn.io/yarn-popular")
			if err != nil {
				handleErr(err)
			}
		} else {
			resp, err = http.Get(fmt.Sprintf("https://getyarn.io/yarn-find?text=%s", q.Text))
			if err != nil {
				handleErr(err)
			}
		}

		clips, err := parseClips(resp.Body)
		if err != nil {
			handleErr(err)
		}

		if len(clips) > 50 {
			clips = clips[0:50]
		}

		results := make(tb.Results, len(clips))
		for i, clip := range clips {
			results[i] = &tb.Mpeg4GifResult{
				ResultBase: tb.ResultBase{
					ID: clip,
				},
				URL:       fmt.Sprintf("https://y.yarn.co/%s_text.mp4", clip),
				ThumbURL:  fmt.Sprintf("https://y.yarn.co/%s_text.gif", clip),
				ThumbMIME: "image/gif",
			}
		}

		if err := b.Answer(q, &tb.QueryResponse{
			Results:   results,
			CacheTime: 60,
		}); err != nil {
			handleErr(err)
		}
	})

	b.Start()
}

func parseClips(r io.Reader) ([]string, error) {
	document, err := html.Parse(r)
	if err != nil {
		return nil, err
	}

	containsClipClass := func(attributes []html.Attribute) bool {
		for _, attribute := range attributes {
			if attribute.Key == "class" && strings.Contains(attribute.Val, "clip") {
				return true
			}
		}

		return false
	}

	getHref := func(attributes []html.Attribute) string {
		for _, attribute := range attributes {
			if attribute.Key == "href" {
				return attribute.Val
			}
		}

		return ""
	}

	var clips []string
	var findClip func(*html.Node)
	findClip = func(node *html.Node) {
		if node.Type == html.ElementNode && containsClipClass(node.Attr) {
			linkNode := node.FirstChild.FirstChild

			href := getHref(linkNode.Attr)
			clips = append(clips, strings.Replace(href, "/yarn-clip/", "", 1))
		}

		for child := node.FirstChild; child != nil; child = child.NextSibling {
			findClip(child)
		}
	}

	findClip(document)

	return clips, nil
}
