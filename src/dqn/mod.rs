use tensorflow::Graph;
use tensorflow::ImportGraphDefOptions;
use tensorflow::Session;
use tensorflow::SessionRunArgs;
use tensorflow::Tensor;

pub struct Dqn {
    input_name: &str;
    output_name: &str;
    graph: Graph;
    session: Session;
}

impl Dqn {
    pub fn new(file_data: &[u8], input_name: &str, output_name: &str) -> Self {
        let mut graph = Graph::new();
        graph.import_graph_def(file_data, &ImportGraphDefOptions::new());
        let session = Session::new(&SessionOptions::new(), &graph)
        Dqn {
            input_name, output_name, graph, session
        }
    }

    pub fn run(&mut self, input_tensor: Tensor) -> Tensor {
        let mut args = SessionRunArgs::new();
        let mut input_op = &self.graph.operation_by_name_required(input_name);
        let mut output_op = &self.graph.operation_by_name_required(output_name);
        args.add_feed(input_op, &tensor);
        let result_token = args.request_fetch(output_op, 0);
        session.run(&mut args);

        let result_tensor = args.fetch(result_token);
        result_tensor
    }
}
