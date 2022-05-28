
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; 
fn main() {
        let mut x = THREE_HOURS_IN_SECONDS;
        println!("The value of x is: {}", x);
        x = 6;
        println!("The value of x is: {}", x);

        //ahora es otra variable con el mismo nombre, 
        //la primera deja de existir, esto se denomona shadowing
        let x =4*x;
        {
            //los brackets crean un scope, por lo que solo se cambia la
            //Var en este scope
            let x = x * 2;
            println!("The value of x in the inner scope is: {}", x);
        }
        //Se muestra la variable x de este scope
        println!("The value of x is: {}", x);

        //shadowing nos permite cambiar el tipo de una variable
        let spaces = "   ";
        println!("spaces text: {}", spaces);
        let spaces = spaces.len();
        println!("spaces len: {}", spaces);
        
    
}
