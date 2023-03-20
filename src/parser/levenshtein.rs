//! A utility module to provide suggestions for possibly misspelt JVM opcodes. Since the assembly
//! language allows a lot of overlap in terms of labels, opcodes, and other syntactic elements,
//! type-checking is necessarily more difficult especially when coupled with error recovery via
//! skipping tokens (panic mode).
//! This module calculates the Levenshtein distance between a (possible) label and a JVM opcode,
//! and given a sufficient probability, provides recommendations of the closest matching JVM
//! opcode.
use std::cmp::{max, min};

const LEVENSHTEIN_THRESHOLD: f64 = 0.50;

const JVM_OPCODES: [&'static str; 204] = [
    "aaload",
    "aastore",
    "aconst_null",
    "aload",
    "aload_0",
    "aload_1",
    "aload_2",
    "aload_3",
    "anewarray",
    "areturn",
    "arraylength",
    "astore",
    "astore_0",
    "astore_1",
    "astore_2",
    "astore_3",
    "athrow",
    "baload",
    "bastore",
    "bipush",
    "caload",
    "castore",
    "checkcast",
    "d2f",
    "d2i",
    "d2l",
    "dadd",
    "daload",
    "dastore",
    "dcmpg",
    "dcmpl",
    "dconst_0",
    "dconst_1",
    "ddiv",
    "dload",
    "dload_0",
    "dload_1",
    "dload_2",
    "dload_3",
    "dmul",
    "dneg",
    "drem",
    "dreturn",
    "dstore",
    "dstore_0",
    "dstore_1",
    "dstore_2",
    "dstore_3",
    "dsub",
    "dup",
    "dup2",
    "dup2_x1",
    "dup2_x2",
    "dup_x1",
    "dup_x2",
    "f2d",
    "f2i",
    "f2l",
    "fadd",
    "faload",
    "fastore",
    "fcmpg",
    "fcmpl",
    "fconst_0",
    "fconst_1",
    "fconst_2",
    "fdiv",
    "fload",
    "fload_0",
    "fload_1",
    "fload_2",
    "fload_3",
    "fmul",
    "fneg",
    "frem",
    "freturn",
    "fstore",
    "fstore_0",
    "fstore_1",
    "fstore_2",
    "fstore_3",
    "fsub",
    "getfield",
    "getstatic",
    "goto",
    "goto_w",
    "i2b",
    "i2c",
    "i2d",
    "i2f",
    "i2l",
    "i2s",
    "iadd",
    "iaload",
    "iand",
    "iastore",
    "iconst_0",
    "iconst_1",
    "iconst_2",
    "iconst_3",
    "iconst_4",
    "iconst_5",
    "iconst_m1",
    "idiv",
    "if_acmpeq",
    "if_acmpne",
    "if_icmpeq",
    "if_icmpge",
    "if_icmpgt",
    "if_icmple",
    "if_icmplt",
    "if_icmpne",
    "ifeq",
    "ifge",
    "ifgt",
    "ifle",
    "iflt",
    "ifne",
    "ifnonnull",
    "ifnull",
    "iinc",
    "iload",
    "iload_0",
    "iload_1",
    "iload_2",
    "iload_3",
    "imul",
    "ineg",
    "instanceof",
    "invokeinterface",
    "invokenonvirtual",
    "invokespecial",
    "invokestatic",
    "invokevirtual",
    "ior",
    "irem",
    "ireturn",
    "ishl",
    "ishr",
    "istore",
    "istore_0",
    "istore_1",
    "istore_2",
    "istore_3",
    "isub",
    "iushr",
    "ixor",
    "jsr",
    "jsr_w",
    "l2d",
    "l2f",
    "l2i",
    "ladd",
    "laload",
    "land",
    "lastore",
    "lcmp",
    "lconst_0",
    "lconst_1",
    "ldc",
    "ldc2_w",
    "ldc_w",
    "ldiv",
    "lload",
    "lload_0",
    "lload_1",
    "lload_2",
    "lload_3",
    "lmul",
    "lneg",
    "loaload",
    "lookupswitch",
    "lor",
    "lrem",
    "lreturn",
    "lshl",
    "lshr",
    "lstore",
    "lstore_0",
    "lstore_1",
    "lstore_2",
    "lstore_3",
    "lsub",
    "lushr",
    "lxor",
    "monitorenter",
    "monitorexit",
    "multianewarray",
    "new",
    "newarray",
    "nop",
    "pop",
    "pop2",
    "putfield",
    "putstatic",
    "ret",
    "return",
    "saload",
    "sastore",
    "sipush",
    "swap",
    "synchronized",
    "tableswitch",
    "wide",
];

fn probability(left: &str, right: &str) -> f64 {
    let llen = left.len();
    let rlen = right.len();
    let len = max(llen, rlen);

    let factor = levenshtein(&left.to_lowercase(), llen, &right.to_lowercase(), rlen);

    1.0f64 - factor as f64 / len as f64
}

fn levenshtein(left: &str, llen: usize, right: &str, rlen: usize) -> i32 {
    let mut dp = vec![vec![0; rlen + 1]; llen + 1];

    for i in 0..=llen {
        dp[i][0] = i;
    }

    for j in 0..=rlen {
        dp[0][j] = j;
    }

    for j in 1..=rlen {
        for i in 1..=llen {
            let insert = dp[i][j - 1] + 1;
            let delete = dp[i - 1][j] + 1;
            let r#match = dp[i - 1][j - 1];
            let mismatch = dp[i - 1][j - 1] + 1;

            if left.chars().nth(i - 1).unwrap() == right.chars().nth(j - 1).unwrap() {
                dp[i][j] = min(min(insert, delete), r#match);
            } else {
                dp[i][j] = min(min(insert, delete), mismatch);
            }
        }
    }

    dp[llen][rlen] as i32
}

/// Return the best match opcode for the given string, if possible.
pub fn find_levenshtein_match(candidate: &str) -> Option<String> {
    let (factor, opcode) = JVM_OPCODES
        .iter()
        .map(|opcode| (probability(opcode, candidate), opcode))
        .max_by(|(f1, _), (f2, _)| f1.total_cmp(f2))
        .unwrap_or((0.0, &""));

    if factor >= LEVENSHTEIN_THRESHOLD {
        Some(opcode.to_string())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jvm_instruction_suggestions() {
        assert_eq!(find_levenshtein_match("ldcc"), Some("ldc".to_string()));
        assert_eq!(find_levenshtein_match("ldx"), Some("ldc".to_string()));
        assert_eq!(find_levenshtein_match("ldax"), Some("ldiv".to_string()));
        assert_eq!(find_levenshtein_match("ldcxxxxxxc"), None);
    }
}
