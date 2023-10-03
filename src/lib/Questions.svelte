<script>
    import { invoke } from '@tauri-apps/api/tauri'

    $: question = ''
    $: decision = ''
    $: started = false
    $: finished = false
    $: questions = []
    $: answers = []

    async function addToLog(answer) {
        let ansTranslate = ''

        switch (answer) {
            case "yes":
                ansTranslate = "Да"
                break;
            case "no":
                ansTranslate = "Нет"
                break;
            case "maybe_yes":
                ansTranslate = "Скорее да"
                break;
            case "maybe_no":
                ansTranslate = "Скорее нет"
                break;
            default:
                return
        }

        questions = [...questions, question]
        answers = [...answers, ansTranslate]
    }

    async function process(answer) {
        if (answer === "start") { started = true }

        await addToLog(answer)

        let response = await invoke("process_answer", { answer })

        question = response.question
        decision = response.decision

        if (decision !== "") { finished = true } else { finished = false }
    }

    async function restart() {
        question = ''
        decision = ''
        questions = []
        answers = []

        await invoke("reset_node")
        await process("start")
    }
</script>

<div class="flex flex-col justify-center items-center h-screen header">
    <h2 class="py-2">{question}</h2>
    <h2 class="py-2">{decision}</h2>

    <div class="flex py-2">
        {#if !started && !finished}
            <button class="btn mr-2" on:click={async () => process("start")}>Начать</button>
        {:else if started && finished}
            <button class="btn mr-2" on:click={async () => restart()}>В начало</button>
        {:else}
            <button class="btn mr-2" on:click={async () => process("yes")}>Да</button>
            <button class="btn mr-2" on:click={async () => process("no")}>Нет</button>
            <button class="btn mr-2" on:click={async () => process("maybe_yes")}>Скорее да</button>
            <button class="btn mr-2" on:click={async () => process("maybe_no")}>Скорее нет</button>
        {/if}
    </div>
    <div class="flex py-2">
        <ol>
            {#each questions as question, i}
                <li>
                    <h6>{question} -> {answers[i]}</h6>
                </li>
            {/each}
        </ol>
    </div>
</div>


<style lang="postcss">
    :global(html) {
        background-color: #202020;
    }
</style>