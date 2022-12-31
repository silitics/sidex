import * as __sidex_types from "@sidex/types"

import { I8 } from "@sidex/types";

const x: I8 = I8.from(3);

function test(x: number) {
    const ZERO = I8.default();

    if (!I8.is(x)) {
        return;
    }
    let y = x;
}