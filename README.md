# rice-doll - yet another dice roller, this time using Rust and Pest

This project's focus is more on representing the state of the dice themselves,
rather than the various ways of interpreting any given roll (though there
are functions to cover that). The intention is to allow more of a "virtual
table" for systems that treat dice as tokens that can be shared around, or
that use pools of them to represent longer-term states.