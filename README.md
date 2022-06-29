# KAMI

Kami is a machine-first human-also-first markup language, designed not for human readability, but for usability as an intermediary between our design desires and a machine's logic. Kami takes inspiration from Pandoc's Markdown flavor, and Textile, the latter of which is my favorite markup language. Its name stands for Katie Ampersand's Markup Instrument. I chose Instrument and not Language just becaues KAMI is a very cute acronym.

Kami files should use the `.km` extension, as in `file.km`.

## Philosophy

Seeing the similarities between Markdown and Kami, you might wonder why I'd bother making this. The reason is simple: Markdown is too human-centric. Of course, there is no one Markdown flavor, but the most popular ones just focus too much on being something you can guess, and not something you can use. The main culprit is commonmark, which I will never forgive, and if your project uses it you should reconsider every choice you have made until you realize what led you to deciding that using commonmark is acceptable, and spend the next couple years doing your best to undo those past mistakes.

The rude bits of that are entirely performative, by the way. Use whatever you want (just please let people mod it).

Kami is designed under the idea that markup languages are NOT like programming languages in the ways that they achieve their goals. A programming language is designed for humans, if it was more machine-centric you'd obtain an assembly language. A markup language should be somewhere in the middle. HTML sucks to use because of how machine-centric it is, but Markdown or Textile can be limiting because they focus too much on human readability and use-cases. In a world of these extremes, Kami tries to stand in the middle.

Human readability is still a goal, but it's a goal that should never limit what can be done. Kami is strictly an intermediate between thoughts and HTML, and is not meant to be read by anyone, which, to me, is what markup languages should strive for.

Kami is ultimately meant to fulfill my needs, it is what I want out of a Markup language, and while it tries to be for general use, it doesn't always care if it isn't. It might be worse for _your_ specific goals, and that's okay. That's why there's so many markup languages.

## Specification

### Bold italians and strong empaths (Bold, Italic, Strong and Emphasis)

Kami distinguishes Bold from Strong and Italic from Emphasis. This is because screen readers care about the distinction. Bold is surrounded with asteriscs (`*`) and Italics are surrounded by underscores (`_`). Strong and emphasis are the same, just doubled (`**` and `__`).

### Hyperlinks

Hyperlinks use markdown format: `[Visible text](destination)`.

### Spans

Spans are to be surrounded with at signs (`@`).

### Inline Code

Inline code is to be surrounded with backticks.

For example, `**text**{#hey}` would be parsed into `<strong id="hey">text</strong>`, and `[link](ampersandia.net){rel="me"}` would be parsed into `<a href="ampersandia.net" rel="me">link</a>`.

### Headers

Headers are done the same way as in Markdown, with sequences of hashtags (`#`).

### Attributes

Everything mentioned here can have id, class, and any HTML attribute you might care about. Simply do this `{#id .class1 .class2 attribute="value"}` after the affected part, without a space in between.

To give attributes to a paragraph simply start the paragraph with an attribute sequence.

### Escaping

Everything can be escaped with backslashes. Backslashes can be escaped, too.
