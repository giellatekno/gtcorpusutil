use std::borrow::Cow;

use memchr::memchr_iter;

/// Remove everything inside `@` symbols inside the str `s`. Returns a borrow
/// of the input string if there are no `@` in the input, otherwise returns
/// clone of the input as a new String, with everything inside pairs of `@`'s
/// (including the `@` themselves) removed.
///
/// ```
/// use gtcorpusutil::remove_ats;
/// assert_eq!(Some("abcghi"), remove_ats("abc@def@ghi").as_deref());
/// assert_eq!(Some(""), remove_ats("@hey@").as_deref());
/// assert_eq!(None, remove_ats("invalid@"));
/// ```
pub fn remove_ats(s: &str) -> Option<Cow<'_, str>> {
    let mut indices = memchr_iter(b'@', s.as_bytes());
    let mut a: usize = 0;

    // find first @, or return borrow of entire s if no @'s are found at all
    let Some(mut b) = indices.next() else {
        return Some(Cow::Borrowed(s));
    };

    // find ending @, or return None if not found to indicate invalid string
    let Some(mut c) = indices.next() else {
        return None;
    };

    let mut out = String::new();
    loop {
        out.push_str(&s[a..b]);

        // advance next starting point to after the ending @
        a = c + 1;

        // find next starting @. if not found, add reminder of s and return
        let Some(next_b) = indices.next() else {
            if a < s.len() {
                out.push_str(&s[a..s.len()]);
            }
            return Some(Cow::Owned(out));
        };
        b = next_b;

        // find next ending @, or return None to indicate invalid string if not found
        let Some(next_c) = indices.next() else {
            return None;
        };
        c = next_c;
    }
}

// PREVIOUS IMPLEMENTATION
/*
pub fn remove_ats1(s: &str) -> Cow<'_, str> {
    let at_positions = s
        .char_indices()
        .filter_map(|(pos, ch)| (ch == '@').then_some(pos as i64));

    let mut it = std::iter::once(-1i64)
        .chain(at_positions)
        .chain(std::iter::once(s.len() as i64));

    // Extract the first two elements from the iterator, to see if we're
    // dealing with an @-less string, and "short curcuit" if we are
    let first = it.next().expect("iterator has at least 2 elements");
    let second = it.next().expect("iterator has at least 2 elements");

    if first == -1 && second as usize == s.len() {
        return Cow::Borrowed(s);
    }

    let mut out = String::new();
    let mut every_other = false;
    // push the first one
    out.push_str(&s[first as usize..second as usize]);
    let mut a = first as usize;

    // Then do the rest
    for el in it {
        if every_other {
            out.push_str(&s[a..el as usize]);
        } else {
            a = (el + 1) as usize;
        }
        every_other = !every_other;
    }

    Cow::Owned(out)
}
*/

struct Pairs<IT>(IT);

fn pairs<IT, T>(it: IT) -> Pairs<IT>
where
    IT: Iterator<Item = T>,
{
    Pairs(it)
}

impl<IT, T> Iterator for Pairs<IT>
where
    IT: Iterator<Item = T>,
{
    type Item = (T, T);

    fn next(&mut self) -> Option<Self::Item> {
        let Some(a) = self.0.next() else {
            return None;
        };

        let Some(b) = self.0.next() else {
            panic!("expecting iterator to always produce 2 items when asking for next chunk");
        };

        Some((a, b))
    }
}

#[allow(dead_code)]
pub fn alternative_implementation(s: &str) -> String {
    let indices = std::iter::once(-1isize)
        .chain(memchr::memchr_iter(b'@', s.as_bytes()).map(|i| i as isize))
        .chain(std::iter::once(s.len() as isize));

    let actual = pairs(indices)
        .map(|(a, b)| ((a + 1) as usize, b as usize))
        .map(|(a, b)| &s[a..b])
        .collect::<String>();

    actual
}

#[cfg(test)]
mod tests {
    use super::remove_ats;
    use std::borrow::Cow;

    #[test]
    fn remove_ats_empty() {
        assert_eq!(remove_ats(""), Some(Cow::Borrowed("")));
    }

    #[test]
    fn remove_ats_no_ats() {
        assert_eq!(remove_ats("a string"), Some(Cow::Borrowed("a string")));
    }

    #[test]
    fn remove_ats_remove_empty_at_beginning() {
        assert_eq!(remove_ats("@@xxx"), Some(Cow::Owned::<str>("xxx".into())));
    }

    #[test]
    fn remove_ats_remove_at_beginning() {
        assert_eq!(
            remove_ats("@hey@xxx"),
            Some(Cow::Owned::<str>("xxx".into()))
        );
    }

    #[test]
    fn remove_ats_remove_in_middle() {
        assert_eq!(remove_ats("a@b@c"), Some(Cow::Owned::<str>("ac".into())));
    }

    #[test]
    fn remove_ats_remove_consecutive_in_middle() {
        assert_eq!(remove_ats("a@b@@c@d"), Some(Cow::Owned::<str>("ad".into())));
    }

    #[test]
    fn remove_ats_remove_at_end() {
        assert_eq!(
            remove_ats("abc@def@"),
            Some(Cow::Owned::<str>("abc".into()))
        );
    }

    #[test]
    fn remove_ats_remove_consecutive_at_end() {
        assert_eq!(
            remove_ats("abc@def@@ghi@"),
            Some(Cow::Owned::<str>("abc".into()))
        );
    }

    #[test]
    fn remove_ats_one_full_string_to_remove() {
        assert_eq!(
            remove_ats("@this is long@"),
            Some(Cow::Owned::<str>("".into()))
        );
    }

    #[test]
    fn remove_ats_consecutive_full_strings_to_remove() {
        assert_eq!(
            remove_ats("@first@@second@"),
            Some(Cow::Owned::<str>("".into()))
        );
    }

    #[test]
    fn remove_ats_beginning_and_end() {
        assert_eq!(
            remove_ats("@first@middle@second@"),
            Some(Cow::Owned::<str>("middle".into()))
        );
    }

    #[test]
    fn remove_ats_return_value_is_borrow_of_input_when_no_ats_in_input() {
        use std::borrow::Borrow;

        let input = "hey";
        let result = remove_ats(input).expect("doesn't contain any @'s, so it's not invalid");

        // it's actually a Cow::Borrowed
        assert!(matches!(result, Cow::Borrowed(_)));

        // and the strings are equal in value...
        assert_eq!(result, input);

        // and in fact, the borrowed string returned from `remove_ats()` is the
        // same string as the input
        let borrowed: &str = result.borrow();
        assert_eq!(borrowed.as_ptr().addr(), input.as_ptr().addr());
    }
}
