#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod tutor_contract {
    // question example
    // What is 1+4*$x+2?
    // 1) (1+4)*$x+2
    // 2) 1+4*($x+2)
    // 3) (1+4)*($x+2)
    // 4*) 1+4*$x+2

    // question = "What is 1+4*3+2?\n  1) 17\n 2) 21\n 3) 30\n4) 15"
    // answer = 4

    use ink::{
        prelude::string::String,
        prelude::vec::Vec,
        prelude::collections::VecDeque,
        env::hash,
    };

    //const SMLY3: Balance = 100000000;
    const SMLY3: Balance = 1;
    type FixedPoint = i64;
    const FP_NOM: f64 = 1000000.0;

    #[ink(storage)]
    pub struct TutorContract {
        questions: Vec<String>,
        answers: Vec<u8>,
        weights: Vec<FixedPoint>,
        points: VecDeque<FixedPoint>, 
        progress: u8,
        grade: FixedPoint,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        TooSmallAmount,
        TooPoor,
        FalseAnswer,
        InternalError,
    }

    #[ink(event)]
    pub struct Answered {
        correct: bool,
    }

    impl TutorContract {
        #[ink(constructor, payable)]
        pub fn new(question: Vec<String>, answers: Vec<u8>) -> Result<Self, Error> {
            let question_clone = question.clone();
            let answers_clone = answers.clone();
            let weights = Vec::from([Self::to_fp(2.0), Self::to_fp(1.5), Self::to_fp(0.90), Self::to_fp(0.4), Self::to_fp(0.2)]);
            let points = VecDeque::from([Self::to_fp(1.0); 5]);
            let grade = Self::dot_product(weights.clone(), points.clone().into());
            Ok(Self {
                questions: question_clone,
                answers: answers_clone,
                weights,
                points,
                progress: 0,
                grade,
            })
        }

        fn to_fp(x: f64) -> FixedPoint {
            return (x*FP_NOM) as FixedPoint;
        }

        fn from_fp(x: FixedPoint) -> f64 {
            return x as f64/FP_NOM;
        }

        fn fp_mul(x: FixedPoint, y: FixedPoint) -> FixedPoint {
            return Self::to_fp((Self::from_fp(x)) * Self::from_fp(y));
        }


        fn dot_product(xs: Vec<FixedPoint>, ys: Vec<FixedPoint>) -> FixedPoint {
            // Calculate the dot product of two vectors.
            assert_eq!(xs.len(), ys.len());
            let mut product: FixedPoint = 0;
            for i in 0..xs.len() {
                product += Self::fp_mul(xs[i], ys[i]);
            }
            return product;
        }
        

        fn calculate_grade(&self) -> FixedPoint {
            return Self::dot_product(self.weights.clone(), self.points.clone().into());
        }

        #[ink(message)]
        pub fn get_current_question(&self) -> String {
            return self.questions[usize::from(self.progress)].clone();
        }

        #[ink(message)]
        pub fn caller(&self) -> AccountId {
            return self.env().caller();
        }

        #[ink(message)]
        pub fn balance(&self) -> Balance {
            return self.env().balance();
        }

        #[ink(message)]
        pub fn grade(&self) -> FixedPoint {
            return self.calculate_grade();
        }

        #[ink(message, payable)]
        pub fn answer(&mut self, answer: u8) -> Result<(), Error> {
            let caller = self.env().caller();


            if self.env().balance() < 500*SMLY3 {
                return Err(Error::TooPoor);
            }

            if self.env().transferred_value() < 100*SMLY3 {
                return Err(Error::TooSmallAmount);
            }

            if answer != self.answers[usize::from(self.progress)] {
                self.points.push_front(Self::to_fp(-2.0));
                self.points.pop_back();
                self.env().emit_event(Answered{correct: false});
                return Err(Error::FalseAnswer);
            }


            self.progress += 1;

            self.points.push_front(Self::to_fp(2.0));
            self.points.pop_back();
            self.env().emit_event(Answered{correct: true});
            
            if usize::from(self.progress) >= self.questions.len() {
                self.progress = 0;
            }

            self.grade = self.calculate_grade();

            if Self::from_fp(self.grade) >= 9.75 {
                match self.env().transfer(caller, 500*SMLY3) {
                    Err(_) => return Err(Error::InternalError),
                    Ok(_) => (),
                }
            }

            return Ok(());
        }

//        #[ink(message)]
//        pub fn random(&mut self, max_value: u8) -> u8 {
//            let seed = self.env().block_timestamp();
//            let mut input: Vec<u8> = Vec::new();
//            input.extend_from_slice(&seed.to_be_bytes());
//            input.extend_from_slice(&self.salt.to_be_bytes());
//            let mut output = <hash::Keccak256 as hash::HashOutput>::Type::default();
//            ink::env::hash_bytes::<hash::Keccak256>(&input, &mut output);
//            self.salt += 1;
//            let number = output[0] % (max_value + 1);
//            number
//        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn answering_works() {
            let questions = Vec::from(["question 0: 0".to_string(), "question 1: 1".to_string()]);
            let answers = Vec::from([0,1]);
            let mut contract = TutorContract::new(questions, answers).unwrap();

            while contract.grade < TutorContract::to_fp(9.75) {
                println!("{}", contract.grade);
                assert_eq!(contract.get_current_question(), "question 0: 0".to_string());
                assert_eq!(Ok(()), ink::env::pay_with_call!(contract.answer(0), 100*SMLY3));
                println!("{}", contract.grade);
                assert_eq!(contract.get_current_question(), "question 1: 1".to_string());
                assert_eq!(Ok(()), ink::env::pay_with_call!(contract.answer(1), 100*SMLY3));
                println!("{}", contract.grade);
            }
            assert_eq!(contract.balance(), 1000900*SMLY3);
        }

        #[ink::test]
        fn multiple_accounts_work() {
            ;
        }
    }

//    #[cfg(all(test, feature = "e2e-tests"))]
//    mod e2e_tests {
//        use super::*;
//        use ink_e2e::build_message;
//
//        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;
//
//        #[ink_e2e::test]
//        async fn it_works(mut client: ink_e2e::Client<C,E>)-> E2EResult<()> {
//            let constructor = TutorContractRef::new("spurning", 1, 45);
//            let contract_acc_id = cilent
//                .instantiate("tutor contract", &ink_e2e::alice(), constructor, 0, None)
//                .await
//                .expect("instantiate failed")
//                .account_id;
//
//            let get = build_message::<TutorContractRef>(contract_acc_id.clone()).call(|tutor| tutor.get_question());
//            let get_res = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
//            assert!(matches!(get_res.return_value(), "spurning"));
//
//
//            let answer = build_message::<TutorContractRef>(contract_acc_id.clone()).call(|tutor| tutor.answer_question(0));
//            let _answer_res = client
//                .call(&ink_e2e::bob(), flip, 0, None)
//                .await
//                .expect("getting failed");
//
//            let correct_answer = build_message::<TutorContractRef>(contract_acc_id.clone()).call(|tutor| tutor.answer_question(1));
//            let correct_answer_res = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
//            assert!(matches!(correct_answer_res.return_value(), Ok(())));
//
//            return Ok(());
//        }
//    }
}

