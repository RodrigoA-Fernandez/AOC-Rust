use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut lista_izquierda = vec![];
    let mut lista_derecha = vec![];
    for l in input.lines() {
        let split: Vec<_> = l.split("   ").collect();
        lista_izquierda.push(split[0].parse::<i32>().unwrap());
        lista_derecha.push(split[1].parse::<i32>().unwrap());
    }

    lista_izquierda.sort();
    lista_derecha.sort();

    let mut valor = 0;
    for i in 0..lista_derecha.len() {
        valor += i32::abs_diff(lista_derecha[i], lista_izquierda[i]);
    }

    Some(valor)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lista_izquierda = vec![];
    let mut mapa_derecha: HashMap<u32, u32> = HashMap::new();
    for l in input.lines() {
        let split: Vec<_> = l.split("   ").collect();
        lista_izquierda.push(split[0].parse::<u32>().unwrap());
        let derecho = split[1].parse::<u32>().unwrap();
        match mapa_derecha.contains_key(&derecho) {
            true => {
                *mapa_derecha.get_mut(&derecho).unwrap() += 1;
            }
            false => {
                mapa_derecha.insert(derecho, 1);
            }
        }
    }
    let mut similarity_score = 0;
    for v in lista_izquierda {
        similarity_score += v * *mapa_derecha.get(&v).unwrap_or(&0);
    }
    Some(similarity_score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
