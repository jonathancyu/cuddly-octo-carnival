//#[cfg(test)]
//mod gradient_tests {
//    use llm_rs::tensor::Tensor;
//
//
//    #[test]
//    fn simple_addition() {
//        let a = Tensor::singleton(1.0);
//        let b = Tensor::singleton(2.0);
//        let c = a + b;
//        println!("{}", *a.grad.borrow());
//
//        let loss = 1.0;
//
//        (c.backward)(c.parents, loss);
//
//        assert_eq!(-1.0, *a.grad.borrow());
//        assert_eq!(-1.0, *b.grad.borrow());
//
//    }
//
//}