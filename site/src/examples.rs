const UIUA: &str = "\"Um, I um...arrays\"\n⊜⊢≥@A.";
const FORMAT: &str = "# Click Run to format!\nrepeat(join∶/+take`2.)10range2";
const D3: &str = "↯∶⇡/×.2_3_4";
pub const LOGO: &str = "\
xy ← ⍘⍉⊞⊟. ÷÷2∶ -÷2,⇡.200
rgb ← [∶⍘⊟×.xy ↯△⊢xy0.5]
u ← ↥<0.2∶>0.7.+×2 ×.∶⍘⊟xy
c ← <∶√/+ⁿ2 xy
⍉⊂∶-¬u c1 +0.1 ∺↧rgb c0.95";
const AVG: &str = "Avg ← ÷⧻∶/+.\nAvg 0_2_1_5";
const CHORD: &str = "\
[0 4 7 10]
×220 ⁿ∶2÷12
÷⧻∶ ≡/+ ○×τ ⊞× ÷∶⇡.&asr.";
pub const QUADRATIC: &str = "\
Quad ← ÷∶+↷∋(×2)¯(⊟¯.√+ⁿ2∶××¯4)
Quad 1 2 0";
const STRIPES: &str = "\
∺(|1 ⊞ ∶,)+_↥_- ⇡300
⍉ ÷2 +1.2 ○ ÷10";
const PALINDROME: &str = r#"$ uiua racecar wow cool!
⍛@ ⊜(⊂⊏∶"❌✅" ≅⇌..)≠@ ."#;
const RULE_30: &str = "\
Thirty ← ≡(↥≅0_1_1 ∶=1 /+.) ◫3 ⊂∶0 ⊂0
size ← 500
start ← =÷2 size ⇡+1 size
⇌[⍥(Thirty.)÷2 size start]";
const PRIMES: &str = "▽¬∊∶♭⊞×...+2⇡50";
const MANDELBROT: &str = "\
Z ← ⊟/- ⁿ2 ∶×2 /×.⇌
⇌⍘⍉⊞⊟.×4 ÷∶-÷2,⇡. 300
<2 √/+ ⁿ2;∶⍥(+Z ∶,)20 ↯∶0△.";
const LIFE: &str = "\
life ← ↥↧↶=2∶=3.-,/+/+⍚1_2↻-1⇡3_3.
⍛0↙10_10 ⋯×4 0_2_4_7
⍉;⍥(∶⊂↶.life)10.
↯∶≡'↯4∵'↯4∶×4△.";

pub const EXAMPLES: &[&str] = &[
    UIUA, FORMAT, D3, LOGO, AVG, CHORD, QUADRATIC, STRIPES, PALINDROME, RULE_30, PRIMES,
    MANDELBROT, LIFE,
];

#[cfg(test)]
#[test]
fn test_examples() {
    use uiua::Uiua;
    for example in EXAMPLES {
        Uiua::with_native_sys()
            .load_str(example)
            .unwrap_or_else(|e| panic!("Example failed:\n{example}\n{e}"));
    }
}
