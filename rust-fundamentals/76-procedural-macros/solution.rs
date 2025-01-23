use proc_macro::TokenStream;
use syn;
use quote::quote;
// use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields, FieldsNamed};

#[proc_macro_derive(Describe)]
pub fn describe_derive(input: TokenStream) -> TokenStream {
    // Parse input token stream.
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);
    let name = &ast.ident;

    // Build a list of fields.
    let fields = match &ast.data {
        syn::Data::Struct(s) => {
            if let syn::Fields::Named(named_fields) = &s.fields {
                &named_fields.named
            } else {
                return TokenStream::new(); // Ignore unnamed fields.
            }
        },
        _ => { return TokenStream::new(); }, // Ignore enums and unions.
    }; 
    let field_names = fields.iter().map(|f| f.ident.as_ref().unwrap());
    let field_names_str = field_names.clone().map(|name| name.to_string());

    // Generate output token stream.    
    let expanded = quote! {
        impl Describe for #name {
            fn describe(&self) -> String {
                let fields: Vec<String> = vec![
                    #(format!("{}: {:?}", #field_names_str, self.#field_names)),*
                ];
                format!("{} {{ {} }}", stringify!(#name), fields.join(", "))
            }
        }
    };
    TokenStream::from(expanded)
}

// Example Test
//#[test]
//fn test_example() {
//    #[derive(Describe)]
//    struct Person {
//        name: String,
//        age: u32,
//    }
//
//    let person = Person {
//        name: "Alice".to_string(),
//        age: 30,
//    };
//
//    assert_eq!(person.describe(), "Person { name: \"Alice\", age: 30 }");
//}

