<script lang="ts">
    import { EditorSelection } from "@codemirror/state";
    import { EditorView } from "@codemirror/view";
    import {
        defaultHighlightStyle,
        syntaxHighlighting,
    } from "@codemirror/language";
    import { xml } from "@codemirror/lang-xml";
    import { onMount } from "svelte";

    let { text = $bindable() }: { text: string } = $props();

    let dom: HTMLElement;
    let view: EditorView | undefined = $state();

    export function focus() {
        view!.dispatch({
            selection: EditorSelection.cursor(view!.state.doc.length),
        });
        view!.focus();
    }

    onMount(() => {
        view = new EditorView({
            doc: text,
            parent: dom,
            extensions: [
                EditorView.lineWrapping,
                EditorView.updateListener.of(
                    (update) => (text = update.state.doc.toString()),
                ),
                syntaxHighlighting(defaultHighlightStyle),
                xml(),
            ],
        });

        return () => view!.destroy();
    });
</script>

<div bind:this={dom}></div>
