use valerie::prelude::components::*;
use valerie::prelude::*;

fn launch_page() -> Node {
    let list: StateVec<StateAtomic<isize>> = StateVec::new();
    let num = StateAtomic::new(0isize);
    let double = StateAtomic::from(&num, |x| x * 2);

    div!(
        div!(
            p!("There are two variable here"),
            p!("Both are states, the second one \'double\' derives its info from the first state"),
            p!("The second value is the double of first value, though we never change the second value"),
            p!("It also increments and decrements the list when you press +1 and -1")
        ),
        button!("first + 1")
            .on_event("click", (num.clone(), list.clone()), |(x, list), _| {
                *x += 1;
                if list.clone().into_iter().find(|y| y.value() == x.value()).is_none() {
                    list.push_atomic(x.value());
                }
            }),
        button!("first - 1")
            .on_event("click", (num.clone(), list.clone()), |(x, list), _| {
                if let Some(pos) = list.clone().into_iter().position(|y| y.value() == x.value()) {
                    list.remove(pos);
                }
                *x -= 1;
            }),
        div!(
            p!("Value: ", num),
            p!("Double: ", double)
        ),
        list.view(ul!(), |x| li!(x))
    ).into()
}

#[valerie(start)]
pub fn run() {
    App::render_single(launch_page());
}
