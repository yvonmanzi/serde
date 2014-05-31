use test::Bencher;

use serialize::{Decoder, Decodable};

use de::{Deserializer, Deserializable, Token};

//////////////////////////////////////////////////////////////////////////////

#[deriving(Clone, PartialEq, Show, Decodable)]
enum Animal {
    Dog,
    Frog(String, int)
}

impl<E, D: Deserializer<E>> Deserializable<E, D> for Animal {
    #[inline]
    fn deserialize_token(d: &mut D, token: Token) -> Result<Animal, E> {
        match try!(d.expect_enum_start(token, "Animal", ["Dog", "Frog"])) {
            0 => {
                try!(d.expect_end());
                Ok(Dog)
            }
            1 => {
                let x0 = try!(Deserializable::deserialize(d));
                let x1 = try!(Deserializable::deserialize(d));

                try!(d.expect_end());

                Ok(Frog(x0, x1))
            }
            _ => d.syntax_error(),
        }
    }
}

//////////////////////////////////////////////////////////////////////////////

#[deriving(Show)]
enum Error {
    EndOfStream,
    SyntaxError,
}

//////////////////////////////////////////////////////////////////////////////

mod decoder {
    use serialize::Decoder;

    use super::{Animal, Dog, Frog, Error, SyntaxError};

    enum State {
        AnimalState(Animal),
        DogState,
        FrogState,
        IntState(int),
        StringState(String),
    }

    pub struct AnimalDecoder {
        stack: Vec<State>,

    }

    impl AnimalDecoder {
        #[inline]
        pub fn new(animal: Animal) -> AnimalDecoder {
            AnimalDecoder {
                stack: vec!(AnimalState(animal)),
            }
        }
    }

    impl Decoder<Error> for AnimalDecoder {
        // Primitive types:
        fn read_nil(&mut self) -> Result<(), Error> { Err(SyntaxError) }
        fn read_uint(&mut self) -> Result<uint, Error> { Err(SyntaxError) }
        fn read_u64(&mut self) -> Result<u64, Error> { Err(SyntaxError) }
        fn read_u32(&mut self) -> Result<u32, Error> { Err(SyntaxError) }
        fn read_u16(&mut self) -> Result<u16, Error> { Err(SyntaxError) }
        fn read_u8(&mut self) -> Result<u8, Error> { Err(SyntaxError) }
        #[inline]
        fn read_int(&mut self) -> Result<int, Error> {
            match self.stack.pop() {
                Some(IntState(x)) => Ok(x),
                _ => Err(SyntaxError),
            }
        }
        fn read_i64(&mut self) -> Result<i64, Error> { Err(SyntaxError) }
        fn read_i32(&mut self) -> Result<i32, Error> { Err(SyntaxError) }
        fn read_i16(&mut self) -> Result<i16, Error> { Err(SyntaxError) }
        fn read_i8(&mut self) -> Result<i8, Error> { Err(SyntaxError) }
        fn read_bool(&mut self) -> Result<bool, Error> { Err(SyntaxError) }
        fn read_f64(&mut self) -> Result<f64, Error> { Err(SyntaxError) }
        fn read_f32(&mut self) -> Result<f32, Error> { Err(SyntaxError) }
        fn read_char(&mut self) -> Result<char, Error> { Err(SyntaxError) }
        #[inline]
        fn read_str(&mut self) -> Result<String, Error> {
            match self.stack.pop() {
                Some(StringState(x)) => Ok(x),
                _ => Err(SyntaxError),
            }
        }

        // Compound types:
        #[inline]
        fn read_enum<T>(&mut self, name: &str, f: |&mut AnimalDecoder| -> Result<T, Error>) -> Result<T, Error> {
            match self.stack.pop() {
                Some(AnimalState(animal)) => {
                    self.stack.push(AnimalState(animal));
                    if name == "Animal" {
                        f(self)
                    } else {
                        Err(SyntaxError)
                    }
                }
                _ => Err(SyntaxError)
            }
        }

        #[inline]
        fn read_enum_variant<T>(&mut self, names: &[&str], f: |&mut AnimalDecoder, uint| -> Result<T, Error>) -> Result<T, Error> {
            let name = match self.stack.pop() {
                Some(AnimalState(Dog)) => "Dog",
                Some(AnimalState(Frog(x0, x1))) => {
                    self.stack.push(IntState(x1));
                    self.stack.push(StringState(x0));
                    "Frog"
                }
                _ => { return Err(SyntaxError); }
            };

            let idx = match names.iter().position(|n| *n == name) {
                Some(idx) => idx,
                None => { return Err(SyntaxError); }
            };

            f(self, idx)
        }
        #[inline]
        fn read_enum_variant_arg<T>(&mut self, _a_idx: uint, f: |&mut AnimalDecoder| -> Result<T, Error>) -> Result<T, Error> {
            f(self)
        }
        fn read_enum_struct_variant<T>(&mut self,
                                       _names: &[&str],
                                       _f: |&mut AnimalDecoder, uint| -> Result<T, Error>)
                                       -> Result<T, Error> { Err(SyntaxError) }
        fn read_enum_struct_variant_field<T>(&mut self,
                                             _f_name: &str,
                                             _f_idx: uint,
                                             _f: |&mut AnimalDecoder| -> Result<T, Error>)
                                             -> Result<T, Error> { Err(SyntaxError) }

        fn read_struct<T>(&mut self, _s_name: &str, _len: uint, _f: |&mut AnimalDecoder| -> Result<T, Error>)
                          -> Result<T, Error> { Err(SyntaxError) }
        fn read_struct_field<T>(&mut self,
                                _f_name: &str,
                                _f_idx: uint,
                                _f: |&mut AnimalDecoder| -> Result<T, Error>)
                                -> Result<T, Error> { Err(SyntaxError) }

        fn read_tuple<T>(&mut self, _f: |&mut AnimalDecoder, uint| -> Result<T, Error>) -> Result<T, Error> { Err(SyntaxError) }
        fn read_tuple_arg<T>(&mut self, _a_idx: uint, _f: |&mut AnimalDecoder| -> Result<T, Error>) -> Result<T, Error> { Err(SyntaxError) }

        fn read_tuple_struct<T>(&mut self,
                                _s_name: &str,
                                _f: |&mut AnimalDecoder, uint| -> Result<T, Error>)
                                -> Result<T, Error> { Err(SyntaxError) }
        fn read_tuple_struct_arg<T>(&mut self,
                                    _a_idx: uint,
                                    _f: |&mut AnimalDecoder| -> Result<T, Error>)
                                    -> Result<T, Error> { Err(SyntaxError) }

        // Specialized types:
        fn read_option<T>(&mut self, _f: |&mut AnimalDecoder, bool| -> Result<T, Error>) -> Result<T, Error> { Err(SyntaxError) }

        #[inline]
        fn read_seq<T>(&mut self, f: |&mut AnimalDecoder, uint| -> Result<T, Error>) -> Result<T, Error> {
            f(self, 3)
        }
        #[inline]
        fn read_seq_elt<T>(&mut self, _idx: uint, f: |&mut AnimalDecoder| -> Result<T, Error>) -> Result<T, Error> {
            f(self)
        }

        fn read_map<T>(&mut self, _f: |&mut AnimalDecoder, uint| -> Result<T, Error>) -> Result<T, Error> { Err(SyntaxError) }
        fn read_map_elt_key<T>(&mut self, _idx: uint, _f: |&mut AnimalDecoder| -> Result<T, Error>) -> Result<T, Error> { Err(SyntaxError) }
        fn read_map_elt_val<T>(&mut self, _idx: uint, _f: |&mut AnimalDecoder| -> Result<T, Error>) -> Result<T, Error> { Err(SyntaxError) }
    }
}

//////////////////////////////////////////////////////////////////////////////

mod deserializer {
    use super::{Animal, Dog, Frog, Error, EndOfStream, SyntaxError};

    use de::Deserializer;
    use de::{Token, Int, String, EnumStart, End};

    enum State {
        AnimalState(Animal),
        IntState(int),
        StringState(String),
        EndState,

    }

    pub struct AnimalDeserializer {
        stack: Vec<State>,
    }

    impl AnimalDeserializer {
        #[inline]
        pub fn new(animal: Animal) -> AnimalDeserializer {
            AnimalDeserializer {
                stack: vec!(AnimalState(animal)),
            }
        }
    }

    impl Iterator<Result<Token, Error>> for AnimalDeserializer {
        #[inline]
        fn next(&mut self) -> Option<Result<Token, Error>> {
            match self.stack.pop() {
                Some(AnimalState(Dog)) => {
                    self.stack.push(EndState);
                    Some(Ok(EnumStart("Animal", "Dog", 0)))
                }
                Some(AnimalState(Frog(x0, x1))) => {
                    self.stack.push(EndState);
                    self.stack.push(IntState(x1));
                    self.stack.push(StringState(x0));
                    Some(Ok(EnumStart("Animal", "Frog", 2)))
                }
                Some(IntState(x)) => {
                    Some(Ok(Int(x)))
                }
                Some(StringState(x)) => {
                    Some(Ok(String(x)))
                }
                Some(EndState) => {
                    Some(Ok(End))
                }
                None => None,
            }
        }
    }

    impl Deserializer<Error> for AnimalDeserializer {
        #[inline]
        fn end_of_stream_error<T>(&self) -> Result<T, Error> {
            Err(EndOfStream)
        }

        #[inline]
        fn syntax_error<T>(&self) -> Result<T, Error> {
            Err(SyntaxError)
        }
    }
}

//////////////////////////////////////////////////////////////////////////////

#[bench]
fn bench_decoder_dog(b: &mut Bencher) {
    b.iter(|| {
        let animal = Dog;

        let mut d = decoder::AnimalDecoder::new(animal.clone());
        let value: Animal = Decodable::decode(&mut d).unwrap();

        assert_eq!(value, animal);
    })
}

#[bench]
fn bench_decoder_frog(b: &mut Bencher) {
    b.iter(|| {
        let animal = Frog("Henry".to_string(), 349);

        let mut d = decoder::AnimalDecoder::new(animal.clone());
        let value: Animal = Decodable::decode(&mut d).unwrap();

        assert_eq!(value, animal);
    })
}

#[bench]
fn bench_deserializer_dog(b: &mut Bencher) {
    b.iter(|| {
        let animal = Dog;

        let mut d = deserializer::AnimalDeserializer::new(animal.clone());
        let value: Animal = Deserializable::deserialize(&mut d).unwrap();

        assert_eq!(value, animal);
    })
}

#[bench]
fn bench_deserializer_frog(b: &mut Bencher) {
    b.iter(|| {
        let animal = Frog("Henry".to_string(), 349);

        let mut d = deserializer::AnimalDeserializer::new(animal.clone());
        let value: Animal = Deserializable::deserialize(&mut d).unwrap();

        assert_eq!(value, animal);
    })
}
