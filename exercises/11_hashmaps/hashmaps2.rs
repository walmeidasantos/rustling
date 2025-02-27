use std::collections::HashMap;
use std::iter::FromIterator;

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
enum Fruit {
    Apple,
    Banana,
    Mango,
    Lychee,
    Pineapple,
}

fn fruit_basket(basket: &mut HashMap<Fruit, u32>) {
    let fruit_kinds = [
        Fruit::Apple,
        Fruit::Banana,
        Fruit::Mango,
        Fruit::Lychee,
        Fruit::Pineapple,
    ];

    for fruit in fruit_kinds.iter() {
        // Se a fruta não estiver no cesto, adicionamos com uma quantidade mínima de 1
        if !basket.contains_key(fruit) {
            basket.insert(fruit.clone(), 1);
        }
    }

    // Verifica o número total de frutas
    let total_fruits: u32 = basket.values().sum();

    // Se o total for menor que 12, adiciona frutas adicionais
    if total_fruits <= 11 {
        // Por exemplo, podemos adicionar bananas até que o total de frutas seja maior que 11
        let current_bananas = basket.get(&Fruit::Banana).unwrap_or(&0);
        let bananas_to_add = 12 - total_fruits - current_bananas;
        if bananas_to_add > 0 {
            basket.insert(Fruit::Banana, current_bananas + bananas_to_add);
        }
    }
}

fn main() {
    // Aqui você pode experimentar, se necessário
}

#[cfg(test)]
mod tests {
    use super::*;

    // Não modifique esta função!
    fn get_fruit_basket() -> HashMap<Fruit, u32> {
        let content = [(Fruit::Apple, 4), (Fruit::Mango, 2), (Fruit::Lychee, 5)];
        HashMap::from_iter(content)
    }

    #[test]
    fn test_given_fruits_are_not_modified() {
        let mut basket = get_fruit_basket();
        fruit_basket(&mut basket);
        assert_eq!(*basket.get(&Fruit::Apple).unwrap(), 4);
        assert_eq!(*basket.get(&Fruit::Mango).unwrap(), 2);
        assert_eq!(*basket.get(&Fruit::Lychee).unwrap(), 5);
    }

    #[test]
    fn at_least_five_types_of_fruits() {
        let mut basket = get_fruit_basket();
        fruit_basket(&mut basket);
        let count_fruit_kinds = basket.len();
        assert!(count_fruit_kinds >= 5);
    }

    #[test]
    fn greater_than_eleven_fruits() {
        let mut basket = get_fruit_basket();
        fruit_basket(&mut basket);
        let count = basket.values().sum::<u32>();
        assert!(count > 11);
    }

    #[test]
    fn all_fruit_types_in_basket() {
        let fruit_kinds = [
            Fruit::Apple,
            Fruit::Banana,
            Fruit::Mango,
            Fruit::Lychee,
            Fruit::Pineapple,
        ];

        let mut basket = get_fruit_basket();
        fruit_basket(&mut basket);

        for fruit_kind in fruit_kinds.iter() {
            let Some(amount) = basket.get(fruit_kind) else {
                panic!("Fruit kind {fruit_kind:?} was not found in basket");
            };
            assert!(*amount > 0);
        }
    }
}
