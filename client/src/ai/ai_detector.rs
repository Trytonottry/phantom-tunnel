use tch::{Tensor, nn, Device};

pub struct DpiDetector {
    model: nn::Sequential,
}

impl DpiDetector {
    pub fn new() -> Self {
        // Загрузка модели (например, обученной на OONI-данных)
        let vs = nn::VarStore::new(Device::Cpu);
        let model = nn::seq()
            .add(nn::linear(&vs.root(), 10, 64, Default::default()))
            .add_fn(|xs| xs.relu())
            .add(nn::linear(&vs.root(), 64, 32, Default::default()))
            .add_fn(|xs| xs.relu())
            .add(nn::linear(&vs.root(), 32, 2, Default::default()));

        Self { model }
    }

    pub fn is_blocked(&self, features: &[f64]) -> bool {
        let tensor = Tensor::of_slice(features).to_device(Device::Cpu);
        let output = self.model.forward(&tensor);
        let pred = output.softmax(-1, tch::Kind::Float);
        pred.get(1).double_value(&[]) > 0.8  // P(блокировка)
    }
}