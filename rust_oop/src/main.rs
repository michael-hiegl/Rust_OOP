mod Auto
{
    pub struct auto
    {
        Reifen: i32,
        Winterreifen: i32,
    }

    impl auto
    {
        pub fn set(Reifen: i32, Winterreifen: i32) -> auto
        {
            if(Reifen>15 && Winterreifen>17)
            {
                return auto{Reifen, Winterreifen};
            }
            else
            {
                return auto{Reifen: 18, Winterreifen: 20};
            }
        }

        pub fn get(&self) -> i32
        {
            return self.Reifen;
        }
    }
}
fn main()
{
    use Auto;

    let mut meinAuto: Auto::auto = Auto::auto::set(14, 20);

    println!("Mein Auto hat {0}-Zoll Sommereifen", meinAuto.get());
}