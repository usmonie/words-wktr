
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Fields};
use syn::__private::TokenStream;

#[proc_macro_derive(HasField, attributes(field))]
pub fn has_field(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let fields = match input.data {
        syn::Data::Struct(s) => match s.fields {
            Fields::Named(fields) => fields,
            _ => panic!("HasField only works with structs with named fields"),
        },
        _ => panic!("HasField only works with structs"),
    };

    let field_checks = fields.named.iter().map(|f| {
        let field_name = f.ident.as_ref().unwrap().to_string();
        quote! {
            if field == #field_name {
                return true;
            }
        }
    });

    let expanded = quote! {
        impl #name {
            pub fn has_field(field: &str) -> bool {
                #(#field_checks)*
                false
            }
        }
    };

    TokenStream::from(expanded)
}


// #[proc_macro_derive(SQLTable)]
// pub fn create_table_macro_derive(input: TokenStream) -> TokenStream {
//     // Парсинг кода из токенов
//     let ast = parse_macro_input!(input as DeriveInput);
//
//     // Получение имени структуры
//     let name = &ast.ident;
//
//     // Обход полей структуры и генерация SQL
//     let mut query = format!("CREATE TABLE {} (\n", name);
//     if let Data::Struct(DataStruct { fields: Fields::Named(fields), .. }) = ast.data {
//         for field in fields.named {
//             let fieldname = field.ident.unwrap();
//             let fieldtype = field.ty;
//
//             // Некоторое примитивное соответствие типов Rust -> SQL
//             let sqltype = match &fieldtype {
//                 Type::Path(p) if p.qself.is_none() => {
//                     let typename = &p.path.segments.first().unwrap().ident;
//                     match typename.to_string().as_str() {
//                         "i32" => "INTEGER",
//                         "String" => "TEXT",
//                         _ => panic!("Unsupported type: {}", typename),
//                     }
//                 }
//                 _ => panic!("Unsupported field: {}: {:?}", fieldname, fieldtype),
//             };
//             query.push_str(&format!("    {} {},\n", fieldname, sqltype));
//         }
//     }
//     query.push_str(");\n");
//
//     // debug SQL
//     println!("{}", query);
//
//     // конвертирование в токены и возвращение
//     TokenStream::from(quote! {
//         impl #name {
//             fn create_sql() -> &'static str {
//                 #query
//             }
//         }
//     })
// }
