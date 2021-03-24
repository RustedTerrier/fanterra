# Fanterra
#### Generate seeded fantasy RPG worlds for a new game every time.

## FAQ

* What is Fanterra?
  * Fanterra is a text based game I'm making with a focus on exploration, so no maps to give everything away.
* What dependencies does Fanterra use?
  * Fanterra uses [rand](https://docs.rs/rand/0.8.3/rand/) for generating random seeds, [ron](https://docs.rs/ron/0.6.4/ron/), and [serde](https://docs.rs/serde/1.0.125/serde/) which are both used for the configuration file,  
 located at ~/.fanterra/init.ron.
* Why do you use BSD-3-Clause?
  * I'm making this to truly learn rust so I'm still a bit of a noob and I want others to be able to learn from this  
 without having to make their project open source as well.  
 I also like to have credit given to me for any code I wrote which is why I didn't use BSD0.
* Is fanterra cross platform?
  * As of now, no. Fanterra uses the HOME environment variable for worlds and configuration which Windows doesn't have by  
 default and I'm too lazy to put in the effort to make it windows compatible. Maybe in the future though...
