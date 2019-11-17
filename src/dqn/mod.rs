#[cfg(test)]
mod test;
use std::error::Error;
use std::result::Result;
use tensorflow::Graph;
use tensorflow::ImportGraphDefOptions;
use tensorflow::Session;
use tensorflow::SessionOptions;
use tensorflow::SessionRunArgs;
use tensorflow::Tensor;
use tensorflow::TensorType;

pub trait Dqn {
    fn run<T: TensorType>(&mut self, input: Tensor<T>) -> Result<Tensor<T>, Box<dyn Error>>;
}

pub struct TensorflowDqn<'a> {
    input_name: &'a str,
    output_name: &'a str,
    graph: Graph,
    session: Session,
}

impl<'a> TensorflowDqn<'a> {
    pub fn new(
        file_data: &[u8],
        input_name: &'a str,
        output_name: &'a str,
    ) -> Result<Self, Box<dyn Error>> {
        let mut graph = Graph::new();
        graph.import_graph_def(file_data, &ImportGraphDefOptions::new())?;
        let session = Session::new(&SessionOptions::new(), &graph)?;
        Ok(TensorflowDqn {
            input_name,
            output_name,
            graph,
            session,
        })
    }
}

impl<'a> Dqn for TensorflowDqn<'a> {
    fn run<T: TensorType>(&mut self, input_tensor: Tensor<T>) -> Result<Tensor<T>, Box<dyn Error>> {
        let mut args = SessionRunArgs::new();
        let input_op = &self
            .graph
            .operation_by_name_required(self.input_name)
            .unwrap();
        let output_op = &self
            .graph
            .operation_by_name_required(self.output_name)
            .unwrap();
        args.add_feed(input_op, 0, &input_tensor);
        let result_token = args.request_fetch(output_op, 0);
        self.session.run(&mut args).unwrap();

        let result_tensor = args.fetch(result_token).unwrap();
        Ok(result_tensor)
    }
}
