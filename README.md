# KAMI

Kami is a machine-first human-also-first markup language, designed not for human readability, but for usability as an intermediary between our design desires and a machine's logic. Kami takes inspiration from Pandoc's Markdown flavor, and Textile, the latter of which is my favorite markup language. Its name stands for Katie Ampersand's Markup Instrument. I chose Instrument and not Language just becaues KAMI is a very cute acronym.

Kami files should use the `.km` extension, as in `file.km`.

## Philosophy

Seeing the similarities between Markdown and Kami, you might wonder why I'd bother making this. The reason is simple: Markdown is too human-centric. Of course, there is no one Markdown flavor, but the ones I've seen just focus too much on being something you can guess and read, and not something you can use. The main culprit is commonmark (though I'm pretty sure it just intends to be documentation for the most common Markdown patterns, and isn't meant to be used), which I will never forgive, and if your project uses it you should reconsider every choice you have made until you realize what led you to deciding that using commonmark is acceptable, and spend the next couple years doing your best to undo those past mistakes.

The rude bits of that are entirely performative, by the way. I understand that the goals of things like commonmark are just different than (and conflicting with) my own goals. Use whatever you think is best for your project, just let people mod it.

Kami is designed under the idea that markup languages are not like programming languages in the ways that they achieve their goals. A programming language is designed for humans, if it was more machine-centric you'd obtain an assembly language. A markup language should be somewhere in the middle. HTML sucks to use because of how machine-centric it is, but Markdown or Textile can be limiting because they focus too much on human readability and use-cases. In a world of these extremes, Kami tries to stand in the middle.

Human readability is still a goal, but it's a goal that should never limit what can be done. Kami is strictly an intermediate between thoughts and HTML, and is not meant to be read by anyone, which, to me, is what markup languages should strive for.

Kami is ultimately meant to fulfill my needs, it is what I want out of a Markup language, and while it tries to be for general use, it doesn't always care if it isn't. It might be worse for _your_ specific goals, and that's okay. That's why there's so many markup languages.

## Specification

### Bold italians and strong empaths (Bold, Italic, Strong and Emphasis)

Kami distinguishes Bold from Strong and Italic from Emphasis. This is because screen readers care about the distinction. Bold is surrounded with asteriscs (`*`) and Italics are surrounded by underscores (`_`). Strong and emphasis are the same, just doubled (`**` and `__`). I used to struggle remembering this (Textile does it), so I came up with the mnemonic you see in the title of this section. I just memorized it along with this sequence `* _ ** __`. Hopefully that can help you too.

### Hyperlinks

Hyperlinks use markdown format: `[Visible text](destination)`.

### Spans

Spans are to be surrounded with at signs (`@`).

### Inline Code

Inline code is to be surrounded with backticks.

### Headers

Headers are done the same way as in Markdown, with sequences of hashtags (`#`).

## Lists

Unordered lists are marked with a `* ` at the beginning of a paragraph. The space is important, and is part of the token. Ordered lists are marked with a `#. ` at the beginning of the paragraph. The space is also part of the token.

### Attributes

Everything mentioned here can have id, class, and any HTML attribute you might care about. Simply do this `{#id .class1 .class2 attribute="value"}` after the affected part, without a space in between. Note that tokens that have spaces as their last character (like in the case of lists) _don't_ get that space removed. They keep that space.

For example, `**text**{#hey}` would be parsed into `<strong id="hey">text</strong>`, and `[link](ampersandia.net){rel="me"}` would be parsed into `<a href="ampersandia.net" rel="me">link</a>`.

To give attributes to a paragraph simply start the paragraph with an attribute sequence. To give attributes to format blocks (like the `<ul> <ol>` parts of lists) just put an attribute sequence before any of the elements of the block, like this:


```
{#id .class)
* list element
```

### Escaping

Everything can be escaped with backslashes. Backslashes can be escaped, too.
