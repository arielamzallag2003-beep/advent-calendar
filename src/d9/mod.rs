use std::cmp::min;
use crate::days::Day;
use std::collections::HashSet;

fn contained_in_borders(point:(i64,i64), borders:&Vec<((i64,i64),(i64,i64))>) -> bool {
    for border in borders {
        //la bordure est sur X
        if border.0.0 == border.1.0 {
            if point.0 == border.0.0 && point.1 >= border.0.1.min(border.1.1)
                && point.1 <= border.0.1.max(border.1.1) {
                return true;
            }
        }
        else if border.0.1 == border.1.1 { //la bordure est sur Y
            if point.1 == border.0.1 && point.0 >= border.0.0.min(border.1.0)
                && point.0 <= border.0.0.max(border.1.0) {
                return true;
            }
        }
    }
    false
}

fn is_point_green(point:(i64,i64), borders:&Vec<((i64,i64),(i64,i64))>) -> bool {
    let mut nb_borders_crossed = 0;
    for border in borders {
        if border.0.0 == border.1.0 {
            let border_x = border.0.0;
            let min_y = border.0.1.min(border.1.1);
            let max_y = border.0.1.max(border.1.1);

            if border_x <= point.0
                && point.1 >= min_y
                && point.1 < max_y
            {
                nb_borders_crossed += 1;
            }
        }
    }
    nb_borders_crossed % 2 == 1
}

fn set_green_tiles(tiles:&Vec<(i64,i64)>, borders: &Vec<((i64,i64),(i64,i64))>) -> HashSet<(i64,i64)> {
    let mut green = HashSet::new();

    for border in borders {
        if border.0.0 == border.1.0 {
            let min_y = border.0.1.min(border.1.1);
            let max_y = border.0.1.max(border.1.1);
            for y in min_y..=max_y {
                green.insert((border.0.0, y));
            }
        } else {
            let min_x = border.0.0.min(border.1.0);
            let max_x = border.0.0.max(border.1.0);
            for x in min_x..=max_x {
                green.insert((x, border.0.1));
            }
        }
    }

    let ((min_x,min_y),(max_x,max_y)) = get_min_max(tiles);

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            if green.contains(&(x, y)) {
                continue;
            }
            if is_point_green((x,y), borders) {
                green.insert((x,y));
            }
        }
    }

    green
}

fn is_rectangle_green(corners:((i64,i64),(i64,i64)), somme:&Vec<Vec<i64>>) -> bool {
    let mut min_X = corners.0.0.min(corners.1.0) as usize;
    let mut min_Y = corners.0.1.min(corners.1.1) as usize;
    let mut max_X = corners.0.0.max(corners.1.0) as usize;
    let mut max_Y = corners.0.1.max(corners.1.1) as usize;

    let value = somme[max_X][max_Y] - somme[min_X-1][max_Y] - somme[max_X][min_Y-1] + somme[min_X-1][min_Y-1];

    value == 0
}

fn get_min_max(tiles:&Vec<(i64,i64)>) -> ((i64,i64),(i64,i64)) {
    let mut min_X = i64::MAX;
    let mut min_Y = i64::MAX;
    let mut max_X = i64::MIN;
    let mut max_Y = i64::MIN;
    for tile in tiles {
        min_X = tile.0.min(min_X);
        min_Y = tile.1.min(min_Y);
        max_X = tile.0.max(max_X);
        max_Y = tile.1.max(max_Y);
    }
    ((min_X,min_Y),(max_X,max_Y))
}

pub struct Day09;

impl Day for Day09 {
    fn titre(&self) -> &'static str { "Movie Theater" }

    //Trouver le rectangle (son aire) dont l'aire est la plus grande possible en respectant le fait
    //que au moins 2 de ses coins opposés sont des tiles rouge (liste données)
    fn partie1(&self, _input: &str) -> String {
        let mut tiles:Vec<(i64,i64)> = Vec::new();
        for line in _input.lines() {
            let mut tuple = line.trim().split(',');
            let a = tuple.next().unwrap().parse::<i64>().unwrap();
            let b = tuple.next().unwrap().parse::<i64>().unwrap();
            tiles.push((a, b));
        }

        //essai d'optimisation
        /*let mut btm_left_X = (i32::MAX,i32::MIN);
        let mut btm_left_Y = (i32::MAX,i32::MIN);
        let mut btm_right_X = (i32::MIN,i32::MIN);
        let mut btm_right_Y = (i32::MIN,i32::MIN);
        let mut top_left_X = (i32::MAX,i32::MAX);
        let mut top_left_Y = (i32::MAX,i32::MAX);
        let mut top_right_X = (i32::MIN,i32::MAX);
        let mut top_right_Y = (i32::MIN,i32::MAX);

        for tile in tiles {
            if tile.0 < btm_left_X.0 { btm_left_X = tile; }
            else if tile.0 == btm_left_X.0 {
                if tile.1 > btm_left_X.1 { btm_left_X = tile; }
            }
        }*/

        let mut max_area = 0;
        //for tile1 in tiles {
        //  for tile2 in tiles {
        //  }
        //}
        //problem ownership
        for i in 0..tiles.len() {
            for j in i+1..tiles.len() {
                let area = ((tiles[i].0 - tiles[j].0).abs() + 1) * ((tiles[i].1 - tiles[j].1).abs() + 1);
                if area > max_area {
                    max_area = area;
                }
            }
        }

        max_area.to_string()
    }

    //Trouver le rectangle (son aire) dont l'aire est la plus grande possible en respectant le fait
    //que au moins 2 de ses coins opposés sont des tiles rouge (liste données) et que toutes les
    //tiles à l'intérieur soient verte
    fn partie2(&self, _input: &str) -> String {
        let mut tiles:Vec<(i64,i64)> = Vec::new();
        let mut borders:Vec<((i64,i64),(i64,i64))> = Vec::new();
        for line in _input.lines() {
            let mut tuple = line.trim().split(',');
            let a = tuple.next().unwrap().parse::<i64>().unwrap();
            let b = tuple.next().unwrap().parse::<i64>().unwrap();
            tiles.push((a, b));
        }

        for i in 1..tiles.len() {
            borders.push((tiles[i-1], tiles[i]));
        }
        borders.push((tiles[tiles.len() - 1], tiles[0]));
        let mut max_area = 0;
        let green = set_green_tiles(&tiles, &borders);

        let mut grille:Vec<Vec<i64>> = Vec::new();
        let ((min_x,min_y),(max_x,max_y)) = get_min_max(&tiles);
        let mut size_x = max_x - min_x + 1;
        let mut size_y = max_y - min_y + 1;
        for x in 0..size_x as usize {
            let mut temp:Vec<i64> = Vec::new();
            for y in 0..size_y as usize {
                if green.contains(&(x as i64,y as i64)) {
                    temp.push(0);
                }
                else {
                    temp.push(1);
                }
            }
            grille.push(temp);
        }

        let mut somme:Vec<Vec<i64>> = Vec::new();
        for x in 0..size_x as usize {
            let mut temp:Vec<i64> = Vec::new();
            for y in 0..size_y as usize {
                let mut value:i64 = -1;
                if x == 0 {
                    if y == 0 {
                        value = grille[x][y];
                    }
                    else {
                        println!("x = {}, y = {}", x, y);
                        value = grille[x][y] + somme[x][y-1];
                    }
                }
                else if y == 0 {
                    value = grille[x][y] + somme[x-1][y];
                }
                else {
                    //println!("x = {}, y = {}", x, y);
                    value = grille[x][y] + somme[x-1][y] + somme[x][y-1] - somme[x-1][y-1];
                }
                temp.push(value);
            }
            somme.push(temp);
        }

        for i in 0..tiles.len() {
            for j in i+1..tiles.len() {
                let area = ((tiles[i].0 - tiles[j].0).abs() + 1) * ((tiles[i].1 - tiles[j].1).abs() + 1);
                if area > max_area{
                    if is_rectangle_green((tiles[i],tiles[j]), &somme) {
                        max_area = area;
                        //println!("change");
                        //println!("x = {}, y = {} / x = {}, y = {}", tiles[i].0, tiles[i].1, tiles[j].0, tiles[j].1);
                    }
                }
            }
        }

        max_area.to_string()
    }
}


#[cfg(test)]
mod tests {
    use crate::d9::Day09;
    use super::*;
    const INPUT: &str = include_str!("../../inputs/day_09_test.txt");
    #[test]
    fn test_partie1() {
        let d = Day09;
        d.partie1(INPUT);
        assert_eq!(d.partie1(INPUT), "50");
    }

    #[test]
    fn test_partie2() {
        let d = Day09;
        assert_eq!(d.partie2(INPUT), "24");
    }
}