Prism.languages.sidex = {
  comment: [
    {
      pattern: /(^|[^\\:])\/\/.*/,
      lookbehind: true,
      greedy: true,
      inside: {
        link: {
          pattern: /\[`[A-Z]\w*`\]/,
          inside: {
            "class-name": /[A-Z]\w*/,
          },
        },
      },
    },
  ],
  attribute: {
    pattern: /#!?\[.*\]/,
    greedy: true,
    alias: "attr-name",
    inside: {
      string: null,
      number: null,
      punctuation: null,
      "class-name": null,
    },
  },
  string: {
    pattern: /b?"(?:\\[\s\S]|[^\\"])*"|b?r(#*)"(?:[^"]|"(?!\1))*"\1/,
    greedy: true,
  },
  number: /-?\b\d+(?:\.\d+)?(?:e[+-]?\d+)?\b/i,
  keyword: [
    /\b(?:opaque|alias|record|variant|fun|interface|import|wrapper|class|with|type|is|instance|derived)\b/,
    /\b(?:bool|string|f(?:32|64)|[ui](?:8|16|32|64)|idx|string|unit|bytes)\b/,
  ],
  punctuation: /(:|::|\?|!|\+)/,
  "class-name": /\b[A-Z]\w*\b/,
  variable: /\w+/,
}

Prism.languages.sidex.attribute.inside.string = Prism.languages.sidex.string
Prism.languages.sidex.attribute.inside.number = Prism.languages.sidex.number
Prism.languages.sidex.attribute.inside["punctuation"] =
  Prism.languages.sidex["punctuation"]
Prism.languages.sidex.attribute.inside["class-name"] =
  Prism.languages.sidex["class-name"]
//Prism.languages.sidex.attribute.inside.variable = Prism.languages.sidex.variable;
