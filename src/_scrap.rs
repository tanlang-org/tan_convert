// #TODO visitor pattern
// #TODO Expr.fold, no need, will get it for free from an Expr iterator.
// #TODO this is the visitor pattern.
// fn traverse<E>(expr: E, f: Rc<dyn Fn(&Expr)>)
// where
//     E: AsRef<Expr>,
// {
//     let expr = expr.as_ref();

//     match expr {
//         Expr::Array(exprs) => {
//             for x in exprs {
//                 // #TODO ARRRGGhhh, Rc!!
//                 traverse(x, f.clone());
//             }
//         }
//         Expr::List(exprs) => {
//             for x in exprs {
//                 // #TODO ARRRGGhhh, Rc!!
//                 traverse(x, f.clone());
//             }
//         }
//         _ => f(expr),
//     }
// }
