/**
VSR - Predkość średnia

Pociąg z miejscowości A do B jedzie z prędkością v1, a wraca z miejscowości B 
do A z prędkością v2. Obliczyć średnią prędkość na całej trasie. 
Uwaga: Dane wejściowe będą tak dobrane, aby wynik był liczba całkowitą.
Wejście

Na wejściu znajduje się dokładnie jedna liczba całkowita t (1<=t<=1000) 
oznaczająca liczbę zestawów danych. W wierszach od 2 do t+1 znajdują się 
dwie liczby całkowite oddzielone spacja v1 oraz v2 (1<=v1,v2<=10000).
Wyjście

Wyjście składa się z t wierszy. W każdym wierszu powinna znaleźć się 
dokładnie jedna liczba całkowita oznaczająca średnią prędkość.
Przykład

Wejście:
2
50 50
60 40

Wyjście:
50
48
*/

fn vsr(v1:u32, v2:u32) -> u32 {
    2*(v1*v2)/(v1+v2)
}

fn main() {
    let stdin = std::io::stdin();
    let mut buffer = String::new();
    stdin.read_line(&mut buffer).expect("err");

    let t:u16 = buffer.trim().parse().unwrap();
    for _ in 0..t{
        buffer.clear();
        stdin.read_line(&mut buffer).expect("err");
        let v: Vec<_> = buffer.split(' ').map(|el|  el.trim().parse::<u32>().unwrap()).collect();
        let (v1,v2) = (v[0], v[1]);
        println!("{}", vsr(v1, v2));
    }

}
