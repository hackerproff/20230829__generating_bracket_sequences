/* 
                2023	08	29


https://contest.yandex.ru/contest/8458/problems/D/

//--------------------------------------------------------------------------


Дано целое число n.
Требуется вывести все правильные скобочные
последовательности длины 2 ⋅ n,
упорядоченные лексикографически
(см. https://ru.wikipedia.org/wiki/Лексикографический_порядок).

В задаче используются только круглые скобки.

Желательно получить решение, которое работает за время,
пропорциональное общему количеству правильных скобочных
последовательностей в ответе,
и при этом использует объём памяти, пропорциональный n.
Формат ввода

Единственная строка входного файла
содержит целое число n, 0 ≤ n ≤ 11
Формат вывода

Выходной файл содержит сгенерированные
правильные скобочные последовательности,
упорядоченные лексикографически.

*/

  //--------------------------------------------------------------------------

#![allow(unused_variables)]
#![allow(dead_code)]

//--------------------------------------------------------------------------

fn print_stack_bracket(stack: &[u8]) {
    for element in stack {
        if *element == (0 as u8) {
            print!("(");
        } else {
            print!(")");
        }
    }
    println!();
}
//--------------------------------------------------------------------------

fn try_open_bracket( 
    max_depth: u64, 
    opened_brckt: u64, 
    stack: &mut std::vec::Vec<u8>,
    closed_brckt: u64
){

    if ((max_depth * 2) as usize) > stack.len() {
    
        if opened_brckt < max_depth {
        
            stack.push(0 as u8);
            try_open_bracket ( max_depth, opened_brckt + 1, stack, closed_brckt);
            stack.pop();
        }
        
        if closed_brckt < opened_brckt {
        
            stack.push(1 as u8);
            try_open_bracket ( max_depth, opened_brckt , stack, closed_brckt + 1);
            stack.pop();
        }
    
    } else {
        print_stack_bracket ( stack );
        return;
    }
    
    //--------------------------------------------------
}  //  fn try_open_bracket
//--------------------------------------------------------------------------

fn main() {
    let mut readed_string = String::new();

    std::io::stdin()
        .read_line(&mut readed_string)
        .expect("ERR stdin");

    let readed_counter: u64 = readed_string.trim()
        .parse()
        .expect(" error parse u64; ");

    if readed_counter == (0 as u64) {
        return;
    }
    
    if readed_counter > (11 as u64) {
        return;
    }
    //--------------------------

    let mut stack_brckts: std::vec::Vec<u8> = std::vec::Vec::new();
        
    try_open_bracket(
        readed_counter, 
        0, 
        &mut stack_brckts,
        0
    );
    println!();
}
