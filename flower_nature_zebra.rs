//mod.rs
mod wellness {
    pub fn run_program() {
         println!("Starting corporate wellness program...");
         employee_wellness();
         nutrition_guidance();
         physical_activity();
         mental_wellness();
         employee_engagement();
         workplace_environment();
         println!("Corporate wellness program complete!");
    }

    fn employee_wellness() {
         println!("Focusing on employee wellness...");
         println!("Providing education on the benefits of healthy practices...");
         println!("Informing employees of available resources...");
    }

    fn nutrition_guidance() {
         println!("Providing nutrition guidance and support...");
         println!("Creating nutritious meal plans...");
         println!("Offering discounts for healthy food...");
    }

    fn physical_activity() {
         println!("Encouraging physical activity...");
         println!("Organizing fitness classes and programs...");
         println!("Creating incentives for those who participate...");
    }

    fn mental_wellness() {
         println!("Focusing on mental wellness...");
         println!("Providing access to mental health resources...");
         println!("Offering stress-management techniques...");
    }

    fn employee_engagement() {
         println!("Promoting employee engagement...");
         println!("Organizing team-building activities...");
         println!("Creating incentives for participation...");
    }

    fn workplace_environment() {
         println!("Focusing on the workplace environment...");
         println!("Providing access to natural light...");
         println!("Encouraging movement and collaboration...");
    }
}

fn main() {
    wellness::run_program();
}