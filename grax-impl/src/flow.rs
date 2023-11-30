use grax_core::node::NodeBalance;

pub struct BalancedNode<N, B> {
    node: N,
    balance: B,
}

impl<N, B> BalancedNode<N, B> {
    pub fn new(node: N, balance: B) -> Self {
        Self { node, balance }
    }
}

impl<N, B> NodeBalance for BalancedNode<N, B> {
    type Balance = B;

    fn balance(&self) -> &Self::Balance {
        &self.balance
    }

    fn balance_mut(&mut self) -> &mut Self::Balance {
        &mut self.balance
    }
}
