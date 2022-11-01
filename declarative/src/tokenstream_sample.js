/*
  struct ThisIsStruct {
    this_is_field_1: String,
  }
*/

const tokenStream = [
  {
    Ident: {
      ident: "struct",
      span: "#4 bytes(32..39)",
    }
  },
  {
    Ident: {
      ident: "ThisIsStruct",
      span: "#4 bytes(32..39)",
    },
  },
  {
    Group: {
      delimiter: "Brace",
      stream: [
        {
          Ident: {
            ident: "this_is_field_1",
            span: "#4 bytes(361..368)",
          },
        },
        {
          Punct: {
            ch: ':',
            spacing: Alone,
            span: "#4 bytes(361..368)",
          },
        },
        {
          Ident: {
            ident: "String",
            span: "#4 bytes(361..368)",
          },
        },
        {
          Punct: {
            ch: ',',
            spacing: Alone,
            span: "#4 bytes(361..368)",
          },
        }
      ], // tokenstream
      span: "#4 bytes(32..39)",
    }
  },
]