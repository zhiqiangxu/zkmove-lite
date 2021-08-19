use crate::error::{RuntimeError, StatusCode, VmResult};
use crate::frame::Frame;
use crate::value::Value;
use bellman::pairing::Engine;

const EVAL_STACK_SIZE: usize = 256;
const CALL_STACK_SIZE: usize = 256;

pub struct EvalStack<E: Engine>(Vec<Value<E>>);

impl<E: Engine> EvalStack<E> {
    pub fn new() -> Self {
        EvalStack(vec![])
    }

    pub fn push(&mut self, value: Value<E>) -> VmResult<()> {
        if self.0.len() < EVAL_STACK_SIZE {
            self.0.push(value);
            Ok(())
        } else {
            Err(RuntimeError::new(StatusCode::StackOverflow))
        }
    }

    pub fn pop(&mut self) -> VmResult<Value<E>> {
        if self.0.len() == 0 {
            Err(RuntimeError::new(StatusCode::StackUnderflow))
        } else {
            Ok(self.0.pop().unwrap())
        }
    }

    pub fn top(&self) -> Option<&Value<E>> {
        self.0.last()
    }
}

pub struct CallStack<E: Engine>(Vec<Frame<E>>);

impl<E: Engine> CallStack<E> {
    pub fn new() -> Self {
        CallStack(vec![])
    }

    pub fn push(&mut self, frame: Frame<E>) -> VmResult<()> {
        if self.0.len() < CALL_STACK_SIZE {
            self.0.push(frame);
            Ok(())
        } else {
            Err(RuntimeError::new(StatusCode::StackOverflow))
        }
    }

    pub fn pop(&mut self) -> VmResult<Frame<E>> {
        if self.0.len() == 0 {
            Err(RuntimeError::new(StatusCode::StackUnderflow))
        } else {
            Ok(self.0.pop().unwrap())
        }
    }

    pub fn top(&mut self) -> Option<&mut Frame<E>> {
        self.0.last_mut()
    }
}
