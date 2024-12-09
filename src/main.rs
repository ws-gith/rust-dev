#![feature(test)]
extern crate test as std_test;

fn main() {
    println!("Hello, world!");
}

#[allow(dead_code)]
const WORD: &'static str = r#"
Lorem ipsum dolor sit amet consectetur adipisicing elit. Quia, eum? Ipsum nam, quasi ipsam nostrum dolorum repellat nobis
dolore, obcaecati natus numquam voluptatum consequatur est aliquid tempore repellendus distinctio inventore temporibus 
unde ullam, aut tenetur. Voluptas, exercitationem! Ab labore in esse quam error, officia ad culpa. Repudiandae veniam 
dolorem, dolorum nemo magnam nisi illo aliquid corporis suscipit eligendi doloribus fuga explicabo recusandae vel quos
illum doloremque voluptate, totam earum adipisci officia rem dolore molestiae molestias. Laboriosam, quam quo. 
Sed deleniti, explicabo, autem blanditiis aut quia eius optio ex debitis odit quidem, sit consequatur quaerat ipsum
commodi sint officiis voluptas vel. Commodi, soluta. Sit, dolorem autem? Quae dicta voluptate vero fugiat, eligendi
dignissimos ratione fuga, enim, esse aut harum molestiae officiis? Eius doloribus tenetur accusantium, similique 
autem itaque aut non deserunt maiores animi nostrum voluptatem, asperiores temporibus aliquid. Voluptate fugiat
facere quisquam debitis dolore architecto nobis deserunt tempore laborum ipsa nulla sed esse totam distinctio
neque iste molestiae ut optio, quo perspiciatis, blanditiis minima fugit tempora magni. Omnis dignissimos nam
dolores repellendus nemo harum ullam repudiandae sunt pariatur, voluptates, laudantium minima quasi autem et
illo est placeat sequi! Laborum doloribus tempora aliquam officiis iure esse, possimus, porro id nulla,
officia asperiores vel sint consectetur consequuntur blanditiis recusandae! Tenetur perspiciatis, magnam
non, consectetur commodi id laboriosam possimus ut earum exercitationem ipsam animi fugiat! Provident modi 
aliquam eligendi laborum magnam reiciendis quis sed nobis deserunt rerum voluptatibus necessitatibus, dolore
soluta et. Illum temporibus at repellat odio repudiandae. Placeat neque non nulla! Odio tempore odit sed
nostrum mollitia architecto reprehenderit eveniet necessitatibus, quo corporis impedit deleniti quis ipsam
earum ipsum voluptatum in, nulla consequuntur illo. Dolor sit dolorem aliquid officia autem aut voluptate
velit doloribus labore dolorum, dicta magni vel inventore enim repellat a
rchitecto veritatis facere quibusdam incidunt alias repudiandae quas similique vitae. Accusantium?
"#;

#[cfg(test)]
mod test {
    use crate::WORD;
    use std::borrow::Cow;
    use std_test::{black_box, Bencher};

    #[bench]
    fn bench_creating_cow(b: &mut Bencher) {
        b.iter(|| black_box(Cow::Borrowed(WORD)));
    }

    #[bench]
    fn bench_creating_string(b: &mut Bencher) {
        b.iter(|| black_box(String::from(WORD)));
    }

    #[bench]
    fn mutating_cow(b: &mut Bencher) {
        b.iter(|| {
            let cow_value = Cow::Borrowed(WORD);
            let mut owned_cow_value = cow_value.into_owned();

            owned_cow_value.push_str(&WORD);
            black_box(owned_cow_value)
        });
    }

    #[bench]
    fn mutating_owned_string(b: &mut Bencher) {
        b.iter(|| {
            let mut owned_value = String::from(WORD);
            owned_value.push_str(&WORD);

            black_box(owned_value)
        });
    }
}
