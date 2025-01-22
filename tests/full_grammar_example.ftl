### Header comment

## Examples from: https://projectfluent.org/fluent/guide/index.html

## Group comment
# Comment

hello = Hello, world!

# $title (String) - The title of the bookmark to remove.
remove-bookmark = Are you sure you want to remove { $title }?

# $title (String) - The title of the bookmark to remove.
remove-bookmark = Really remove { $title }?

-brand-name = Firefox
installing = Installing { -brand-name }.

opening-brace = This message features an opening curly brace: {"{"}.
closing-brace = This message features a closing curly brace: {"}"}.

blank-is-removed =     This message starts with no blanks.
blank-is-preserved = {"    "}This message starts with 4 spaces.

leading-bracket =
    This message has an opening square bracket
    at the beginning of the third line:
    {"["}.

attribute-how-to =
    To add an attribute to this messages, write
    {".attr = Value"} on a new line.
    .attr = An actual attribute (not part of the text value above)

# This is OK, but cryptic and hard to read and edit.
literal-quote1 = Text in {"\""}double quotes{"\""}.

# This is preferred. Just use the actual double quote character.
literal-quote2 = Text in "double quotes".

privacy-label = Privacy{"\u00A0"}Policy

# The dash character is an EM DASH but depending on the font face,
# it might look like an EN DASH.
which-dash1 = It's a dash—or is it?

# Using a Unicode escape sequence makes the intent clear.
which-dash2 = It's a dash{"\u2014"}or is it?

# This will work fine, but the codepoint can be considered
# cryptic by other translators.
tears-of-joy1 = {"\U01F602"}

# This is preferred. You can instantly see what the Unicode
# character used here is.
tears-of-joy2 = 😂

single = Text can be written in a single line.

multi = Text can also span multiple lines
    as long as each new line is indented
    by at least one space.

block =
    Sometimes it's more readable to format
    multiline text as a "block", which means
    starting it on a new line. All lines must
    be indented by at least one space.

leading-spaces =     This message's value starts with the word "This".
leading-lines =


    This message's value starts with the word "This".
    The blank lines under the identifier are ignored.

blank-lines =

    The blank line above this line is ignored.
    This is a second line of the value.

    The blank line above this line is preserved.

multiline1 =
    This message has 4 spaces of indent
        on the second line of its value.

# █ denotes the indent common to all lines (removed from the value).
# · denotes the indent preserved in the final value.
multiline1 =
████This message has 4 spaces of indent
████····on the second line of its value.

multiline2 =
████··This message starts with 2 spaces on the first
████first line of its value. The first 4 spaces of indent
████are removed from all lines.

multiline3 = This message has 4 spaces of indent
████····on the second line of its value. The first
████line is not considered indented at all.

# Same value as multiline3 above.
multiline4 =     This message has 4 spaces of indent
████····on the second line of its value. The first
████line is not considered indented at all.

multiline5 = This message ends up having no indent
████████on the second line of its value.

welcome = Welcome, { $user }!
unread-emails = { $user } has { $email-count } unread emails.

# $duration (Number) - The duration in seconds.
time-elapsed = Time elapsed: { $duration }s.

# $duration (Number) - The duration in seconds.
time-elapsed = Time elapsed: { NUMBER($duration, maximumFractionDigits: 0) }s.

menu-save = Save
help-menu-save = Click { menu-save } to save the file.

-brand-name = Firefox
installing = Installing { -brand-name }.
