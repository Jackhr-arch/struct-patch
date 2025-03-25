use std::collections::HashMap;

use struct_patch::Patch;

#[derive(PartialEq, Debug, Default)]
struct Custom {
    inner: Vec<u8>,
}

impl std::iter::Extend<u8> for Custom {
    fn extend<T: IntoIterator<Item = u8>>(&mut self, iter: T) {
        self.inner.extend(iter);
    }
}

impl IntoIterator for Custom {
    type Item = u8;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

type Map = HashMap<u8, HashMap<u8, u8>>;

#[derive(struct_patch::Patch, Default, Debug, PartialEq)]
struct Foo {
    #[patch(extendable, skip_if = vec_gq_4)]
    vec: Vec<u8>,
    #[patch(extend = extend_map)]
    map: Map,
    #[patch(extendable)]
    custom: Custom,
    overwrite: String,
    #[patch(skip_if = Option::is_some)]
    keep: Option<String>,
}

macro_rules! test {
    ($a:expr) => {
        assert!($a(&mut Some(1)));
    };
}

fn vec_gq_4(a: &Vec<u8>) -> bool {
    a.len() >= 4
}

fn extend_map(a: &mut Map, b: Map) {
    for (idx, ele) in b.into_iter() {
        a.entry(idx).or_default().extend(ele);
    }
}

fn main() {
    test!(Option::is_some);
    let mut foo = Foo::default();
    let mut patch = Foo::new_empty_patch();
    patch.vec = Some(vec![1, 2, 3]);
    patch.map = Some([(0, [(0, 2)].into_iter().collect())].into_iter().collect());
    patch.custom = Some(Custom { inner: vec![1, 2] });
    foo.apply(patch);
    assert_eq!(
        foo,
        Foo {
            vec: vec![1, 2, 3],
            map: [(0, [(0, 2)].into_iter().collect())].into_iter().collect(),
            custom: Custom { inner: vec![1, 2] },
            overwrite: "".to_owned(),
            keep: None
        }
    );

    let mut patch = Foo::new_empty_patch();
    patch.vec = Some(vec![4, 5]);
    patch.map = Some([(0, [(1, 3)].into_iter().collect())].into_iter().collect());
    patch.custom = Some(Custom { inner: vec![3, 4] });
    patch.overwrite = Some("T1".to_owned());
    patch.keep = Some(Some("T1".to_owned()));
    foo.apply(patch);
    assert_eq!(
        foo,
        Foo {
            vec: vec![1, 2, 3, 4, 5],
            map: [(0, [(0, 2), (1, 3)].into_iter().collect())]
                .into_iter()
                .collect(),
            custom: Custom {
                inner: vec![1, 2, 3, 4]
            },
            overwrite: "T1".to_owned(),
            keep: Some("T1".to_owned())
        }
    );

    let mut patch = Foo::new_empty_patch();
    // greater than 4, this won't apply
    patch.vec = Some(vec![6]);
    patch.map = Some([(1, [(1, 3)].into_iter().collect())].into_iter().collect());
    patch.custom = Some(Custom { inner: vec![5, 6] });
    patch.overwrite = Some("T2".to_owned());
    patch.keep = Some(Some("T2".to_owned()));
    foo.apply(patch);
    assert_eq!(
        foo,
        Foo {
            vec: vec![1, 2, 3, 4, 5],
            map: [
                (0, [(0, 2), (1, 3)].into_iter().collect()),
                (1, [(1, 3)].into_iter().collect())
            ]
            .into_iter()
            .collect(),
            custom: Custom {
                inner: vec![1, 2, 3, 4, 5, 6]
            },
            overwrite: "T2".to_owned(),
            keep: Some("T1".to_owned())
        }
    );
}
