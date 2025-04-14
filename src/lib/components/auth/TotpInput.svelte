<script lang="ts">

    /**
	 * @type {number[] | null[]}
	 */
    export let totp: (number | null)[];
    export let disabled: boolean;
    export let completeTotp: () => void;
</script>

<div class="flex flex-row w-[250px] justify-between">
    {#each [0, 1, 2, 3, 4, 5] as index}
        <!-- svelte-ignore a11y_autofocus -->
        <input
            type="number"
            class="border-[1.5px] border-gray-300 rounded-md opacity-75"
            style="padding: 5px 5px 5px 7.5px; width: 30px; height: 40px;"
            maxlength="1"
            max="9"
            id="totp-{index}"
            disabled={disabled}
            autocomplete="one-time-code"
            bind:value={totp[index]}
            on:input={() => {
                if (totp[index] != null && totp[index] > 9) {
                    const code = String(totp[index]);
                    if (code.length == 6) {
                        totp[0] = parseInt(code.charAt(0));
                        totp[1] = parseInt(code.charAt(1));
                        totp[2] = parseInt(code.charAt(2));
                        totp[3] = parseInt(code.charAt(3));
                        totp[4] = parseInt(code.charAt(4));
                        totp[5] = parseInt(code.charAt(5));
                        completeTotp();
                    } else {
                        totp[index] = parseInt(code.charAt(index));
                    }
                }
                
                if (totp[index] == null && index > 0) {
                    document.getElementById(`totp-${index - 1}`)?.focus();
                } else if (totp[index] != null && index < 5) {
                    document.getElementById(`totp-${index + 1}`)?.focus();
                } else if (totp[index] != null && index == 5) {
                    document.getElementById(`totp-${index}`)?.blur();
                    completeTotp();
                }
            }}
        >
    {/each}
</div>

<style>
/* Hide spin buttons in Chrome */
    input::-webkit-outer-spin-button,
    input::-webkit-inner-spin-button {
        -webkit-appearance: none;
        margin: 0;
    }

    /* Hide spin buttons in Firefox */
    input[type="number"] {
        -moz-appearance: textfield;
    }

    input:focus {
        outline: none;
        border: solid 1.5px var(--color-blue-500);
    }
</style>