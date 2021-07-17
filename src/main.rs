use rumdr::config::get_theme;

fn main() {
    // Temporary markdown string for development.
    let text = String::from("# This is Markdown.
Here's a regular line
* list
* list
- still a list item
* list
*italics?*
# One more header
#Not a header
## And level two!
### This is level three
#### There's no level four so this should be invalid
##This is also invalid
> block quote
> still block quote

> skipped a line!
```
yeet
# This shouldn't count
```");

    let theme = get_theme("default");

    rumdr::run(text, theme);
}
