<script>
    import { invoke } from '@tauri-apps/api/tauri'

    $: question = ''
    $: decision = ''
    $: started = false
    $: finished = false

    async function process(answer) {
        if (answer === "start") { started = true }

        let response = await invoke("process_answer", { answer })

        question = response.question
        decision = response.decision

        if (decision !== "") { finished = true } else { finished = false }
    }

    async function restart() {
        await invoke("reset_node")
        await process("start")
    }
</script>

<div class="flex flex-col justify-center items-center h-screen header">
    <h2 class="py-2">{question}</h2>
    <h2 class="py-2">{decision}</h2>

    <div class="flex py-2">
        {#if !started && !finished}
            <button class="btn mr-2" on:click={async () => process("start")}>start</button>
        {:else if started && finished}
            <button class="btn mr-2" on:click={async () => restart()}>restart</button>
        {:else}
            <button class="btn mr-2" on:click={async () => process("yes")}>yes</button>
            <button class="btn" on:click={async () => process("no")}>no</button>
        {/if}
    </div>
</div>


<style lang="postcss">
    :global(html) {
        background-color: #202020;
    }
</style>