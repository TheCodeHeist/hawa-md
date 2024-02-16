pub enum TokenType {
    HTML,
    SpecialCharacter,

    // Block Elements
    Header,
    Paragraph,
    Blockquote,
    Codeblock,

    UnorderedList,
    OrderedList,
    CheckboxList,

    HorizonalRule,
    LineBreak,

    // Span Elements
    Links,
    AutomaticLink,
    Images,
    Code,
    Latex,

    Bold,
    Italic,
    Underline,
    Strikethrough,

    // Filters
    Whitespace,
    Newline,
    Escape,
    EOF,
}
