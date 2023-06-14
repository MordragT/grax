// no more sink and source (st) -> no st-flow
// b (balance)
// b-flow: sum of all b-flow in the graph is 0
// edge cannot carry more flow than its capacity
// the difference in flow entering and leaving a node
// must be equal b(v) (flow balance)

// wenn capacity von edge == 0 -> edge ignorieren
// supply und demand nodes
// wenn balance < 0 demand node, wenn balance > 0 supply node
// flow kann nicht mehr als die capacities sein aber auch nicht weniger als 0
// differenz zwischen flow der raus geht und reingeht muss gleich dem supply oder demand sein
// wenn flow durch nodes geht, dann muss bei demand-nodes der demand vom flow abgezogen werden,
// analog bei supply-nodes wird der supply addiert.
// möglicherweise nicht solvable wenn im netzwerk weniger supply als demand vorhanden ist, oder mehr supply als demand
// wir können checken ob MCF möglich wenn wir das problem in ein max flow problem überführen

#[cfg(test)]
mod test {
    #[test]
    fn cycle_canceling_kostenminimal_1() {}

    #[test]
    fn cycle_canceling_kostenminimal_2() {}

    #[test]
    fn cycle_canceling_kostenminimal_3() {}

    #[test]
    fn cycle_canceling_kostenminimal_gross_1() {}

    #[test]
    fn cycle_canceling_kostenminimal_gross_2() {}

    #[test]
    fn cycle_canceling_kostenminimal_gross_3() {}
}
