// the following (left for reference) is what the [contract_impl] macro would logically add
// to this code

// impl Routing for AssetContract {    
   
//     fn route2(&self, ctx: Context, tx_fn: String, args: Vec<String>) -> Result<String,String>{
//         ctx.log(format!("Inside the contract {} {:?}",tx_fn,args));
//         let _r = match &tx_fn[..] {
//             "create_asset" =>  {
                
//                 let a0 = match args.get(0) {
//                     Some(a) => Ok(a),
//                     None => Err(String::from("Missing argument 0")),
//                 };
                
//                 let a1 = match args.get(1) {
//                     Some(a) => Ok(a),
//                     None => Err(String::from("Missing argument 1")),
//                 };
               
//                 let _r=self.create_asset(ctx, a0.unwrap().to_string(), a1.unwrap().to_string());
//                 Ok(String::from(""))
//             },
//             "read_asset" =>  {

//                 let a0 = match args.get(0) {
//                     Some(a) => Ok(a),
//                     None => Err(String::from("Missing argument 0")),
//                 };

//                 let _r=self.read_asset(ctx, a0.unwrap().to_string());
//                 Ok(String::from(""))
//             },
//             _ => Err(String::from("Unknown transaction fn "))
//         };

//         Ok(String::from("200"))
//     }
// }