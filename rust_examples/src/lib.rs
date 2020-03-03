

pub mod random_generator {

    pub fn linear_gen(SEED: usize) -> usize{
   
        const A:usize = 84589;
        const B:usize = 45989;
        const M:usize = 217728;
        
        let x = (A * SEED -1 + B ) % M;
        return x
    }
    
    pub fn linear_gen_range(range: usize,mut seed:usize) -> Vec<usize>{
        
        let mut random_numbers: Vec<usize> = Vec::new();
        let mut counter  = 0;
    
        while counter <= range{
            random_numbers.push(linear_gen(seed));
            seed = random_numbers[counter];
            counter += 1;
        }
        return random_numbers;
    }
    
    pub struct linear_feedback_shift{
        start: usize,
    }

    impl Iterator for linear_feedback_shift{
        type Item = usize;

    
    fn next(&mut self) -> Option<Self::Item>{
        //Primitive polynomial mod 2 make the tap seq, x^64 + x^4 + x^3 +x^1
        //bits at position 64,4,3 and 1 are xor in the feedback function
        

        let output_bit = ((self.start >> 0)^
                           (self.start >> 2)^
                           (self.start >> 3)^
                           (self.start >> 63)) & 1;
        self.start =  (self.start >> 1) | (output_bit << 63);
        Some(output_bit)
        
    }
}
pub fn new_linear_feedback_shift(seed: usize) -> linear_feedback_shift{
    linear_feedback_shift {start: seed}
}
}