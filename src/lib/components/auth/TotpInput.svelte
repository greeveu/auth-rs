<script lang="ts">
	import { onMount } from "svelte";


    /**
	 * @type {number[] | null[]}
	 */
    export let totp: (string | null)[];
    export let disabled: boolean;
    export let completeTotp: (code: string) => Promise<boolean>;

    function clearInpts() {
        for (let i = 0; i < totp.length; i++) {
            totp[i] = null;
        }
    }

    onMount(() => {
        document.addEventListener("keydown", (event) => {
            if (event.key === "Backspace") {
                const focusedElement = document.activeElement;
                if (focusedElement && focusedElement.tagName === "INPUT") {
                    const currentIndex = parseInt(focusedElement.id.split("-")[1]);
                    if (totp[currentIndex] == null || totp[currentIndex] == "") {
                        if (currentIndex > 0) {
                            totp[currentIndex - 1] = null;
                            document.getElementById(`totp-${currentIndex - 1}`)?.focus();
                        }
                    }
                }
            }
        });
    })
</script>

<div class="flex flex-row w-[250px] justify-between">
    {#each [0, 1, 2, 3, 4, 5] as index}
        <!-- svelte-ignore a11y_autofocus -->
        <input
            type="text"
            class="border-[1.5px] border-gray-300 rounded-md opacity-75"
            style="padding: 5px 5px 5px 7.5px; width: 30px; height: 40px;"
            max="9"
            id="totp-{index}"
            disabled={disabled}
            autocomplete="one-time-code"
            bind:value={totp[index]}
            on:input={async () => {
                if (totp[index] != null && totp[index].length == 6) {
                    const success = await completeTotp(totp[index]);
                    if (!success) {
                        clearInpts();
                        document.getElementById('totp-0')?.focus();
                    }
                } else if (totp[index] != null) {
                    totp[index] = totp[index].charAt(0);
                }
                
                if (totp[index] != "" && index < 5) {
                    document.getElementById(`totp-${index + 1}`)?.focus();
                } else if (totp[index] != "" && index == 5) {
                    document.getElementById(`totp-${index}`)?.blur();
                    const result = await completeTotp(totp.join(''));
                    if (!result) {
                        clearInpts();
                        document.getElementById('totp-0')?.focus();
                    }
                }
                
            }}
        >
    {/each}
</div>

<style>
    input:focus {
        outline: none;
        border: solid 1.5px var(--color-blue-500);
    }
</style>