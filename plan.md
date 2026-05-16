# plan

## "how about instantiating a struct and calling a method on it"

    There are two main approaches:                                                                                     
                                                                                                                         
      1. Owned value — use the steel_derive crate and #[derive(Steel)]:                                                  
      use steel_derive::Steel;                                                                                           
                                                                                                                         
      #[derive(Steel, Clone, Debug)]                                                                                     
      struct Point { x: f64, y: f64 }                                                                                    
      That auto-derives IntoSteelVal/FromSteelVal, so you can register a constructor and methods directly with           

      2. External reference — implement CustomReference:
      For when you have an existing Rust object you want to pass into the VM without cloning. More involved — uses       
      run_with_reference.                                                                                                
      For your debugging use case, option 1 is the right fit. However I need to check whether steel_derive is a separate 
      crate you need to add to Cargo.toml, or whether it re-exports from steel-core. Do you want me to look into that, or
       try building with steel_derive as a separate dep?


    The natural fit is a #[test] marked #[ignore] — skipped during normal cargo test, but you invoke it     
      explicitly when you want a session:
                                                                                                                         
      cargo test repl -- --ignored --nocapture                                                                           
                                                                                                                         
      The --nocapture is needed so stdin/stdout aren't swallowed by the test harness. You'd put the REPL loop and all    
      your registered types/functions inside that one test, in a #[cfg(test)] module in whatever file has the types you
      care about.

## "could types etc be annotated inside a test module rather than the main body of the codebase and from different library files within the project"


    There are a few constraints worth knowing upfront:                                                        

      What you can do entirely in test code:                                                                             
      - Define new types inside #[cfg(test)] (debug-only wrappers, helpers)
      - Write impl IntoSteelVal for MyType inside #[cfg(test)] in the same file as MyType — Rust allows this, and since  
      MyType is yours the orphan rules are satisfied                                                                   
      - steel-core as a [dev-dependencies] only, so it never ships                                                       
                                                                  
      What requires touching the type definition:                                                                        
      - #[derive(Steel)] has to be at the definition site, so you'd need #[cfg_attr(test, derive(Steel))] on the struct  
      itself — one line per type, but it is touching the main codebase                                                   
                                                                                                                         
      Across multiple files in the same crate:                                                                           
      - Each file's #[cfg(test)] module can implement steel traits for types defined in that file                        
      - A single integration test in tests/ can import types from anywhere in the crate and write impls there            
                                                                                                                         
      The practical split:                                                                                               
      - Types you want to pass by value: #[cfg_attr(test, derive(Steel))] at the definition — minimal intrusion          
      - Types you want to pass by reference (option 2): no annotation needed at all, just impl CustomReference in the    
      test module



### end
