<script lang="ts">
    import {
        ChangeSet,
        EditorSelection,
        EditorState,
        RangeSet,
        RangeValue,
        StateField,
        Transaction,
        type ChangeSpec,
        type Range,
    } from "@codemirror/state";
    import {
        Decoration,
        EditorView,
        keymap,
        ViewPlugin,
        ViewUpdate,
        WidgetType,
        type DecorationSet,
    } from "@codemirror/view";
    import {
        defaultHighlightStyle,
        syntaxHighlighting,
        syntaxTree,
    } from "@codemirror/language";
    import { xml } from "@codemirror/lang-xml";
    import { defaultKeymap } from "@codemirror/commands";
    import { treeView } from "@overleaf/codemirror-tree-view";
    import { mount, onMount, unmount } from "svelte";
    import TagWidget from "./TagWidget.svelte";
    import type { TextStyle } from "./document.svelte";

    let {
        text = $bindable(),
        initialPosition,
        style = "P",
        onSwitchStyle,
    }: {
        text: string;
        initialPosition: number;
        style: TextStyle;
        onSwitchStyle?: (style: TextStyle) => void;
    } = $props();

    let dom: HTMLElement;
    let view: EditorView | undefined = $state();

    export function focus() {
        if (view === undefined) return;
        view.dispatch({
            selection: EditorSelection.cursor(view.state.doc.length),
        });
        view.focus();
    }

    class TagWidgetType extends WidgetType {
        component: Record<string, any> | undefined;
        kind: "Bold" | "Italic";
        isClosing: boolean;

        constructor(kind: "Bold" | "Italic", isClosing: boolean) {
            super();
            this.kind = kind;
            this.isClosing = isClosing;
        }

        toDOM(view: EditorView): HTMLElement {
            const span = document.createElement("span");
            this.component = mount(TagWidget, {
                target: span,
                props: { kind: this.kind, isClosing: this.isClosing },
            });
            return span;
        }

        destroy(dom: HTMLElement): void {
            if (this.component !== undefined) {
                unmount(this.component);
                this.component = undefined;
            }
        }
    }

    function tagDecorations(view: EditorView): DecorationSet {
        let widgets: Range<Decoration>[] = [];
        for (let { from, to } of view.visibleRanges) {
            syntaxTree(view.state).iterate({
                from,
                to,
                enter: (node) => {
                    if (
                        node.name === "OpenTag" ||
                        node.name === "CloseTag" ||
                        node.name === "MismatchedCloseTag"
                    ) {
                        const nameNode = node.node.getChild("TagName");
                        if (
                            nameNode === null ||
                            node.node.getChild("EndTag") === null
                        )
                            return;
                        const name = view.state.sliceDoc(
                            nameNode.from,
                            nameNode.to,
                        );
                        let kind: "Bold" | "Italic";
                        if (name === "b") {
                            kind = "Bold";
                        } else if (name === "i") {
                            kind = "Italic";
                        } else return;

                        let deco = Decoration.replace({
                            widget: new TagWidgetType(
                                kind,
                                node.name !== "OpenTag",
                            ),
                        });
                        widgets.push(deco.range(node.from, node.to));
                    }
                },
            });
        }
        return Decoration.set(widgets);
    }

    const tagDecorationsPlugin = ViewPlugin.fromClass(
        class {
            decorations: DecorationSet;

            constructor(view: EditorView) {
                this.decorations = tagDecorations(view);
            }

            update(update: ViewUpdate) {
                if (
                    update.docChanged ||
                    update.viewportChanged ||
                    syntaxTree(update.startState) != syntaxTree(update.state)
                )
                    this.decorations = tagDecorations(update.view);
            }
        },
        {
            decorations: (v) => v.decorations,
            provide: (plugin) => {
                return [
                    EditorView.atomicRanges.of((view) => {
                        return (
                            view.plugin(plugin)?.decorations || Decoration.none
                        );
                    }),
                ];
            },
        },
    );

    class FormattingHighlightRange extends RangeValue {
        value: "Bold" | "Italic";

        constructor(value: "Bold" | "Italic") {
            super();
            this.value = value;
        }

        eq(other: RangeValue): boolean {
            return (
                other instanceof FormattingHighlightRange &&
                this.value === other.value
            );
        }
    }

    const formattingHighlights = StateField.define<
        RangeSet<FormattingHighlightRange>
    >({
        create(state) {
            const newValue: Range<FormattingHighlightRange>[] = [];
            const active: ["Bold" | "Italic", number][] = [];

            syntaxTree(state).iterate({
                enter: (node) => {
                    let open: boolean;
                    if (node.name === "OpenTag") {
                        open = true;
                    } else if (node.name === "CloseTag") {
                        open = false;
                    } else return;

                    const nameNode = node.node.getChild("TagName");
                    if (nameNode === null) return;
                    const name = state.sliceDoc(nameNode.from, nameNode.to);
                    let kind: "Bold" | "Italic";
                    if (name === "b") {
                        kind = "Bold";
                    } else if (name === "i") {
                        kind = "Italic";
                    } else return;

                    if (open) {
                        active.push([kind, node.to]);
                    } else if (active.length > 0) {
                        const [otherKind, from] = active[active.length - 1];
                        if (kind === otherKind) {
                            active.pop();
                            newValue.push({
                                from,
                                to: node.from,
                                value: new FormattingHighlightRange(kind),
                            });
                        }
                    }
                },
            });
            return RangeSet.of(newValue.toSorted((a, b) => a.from - b.from));
        },
        update(value, tr) {
            if (!tr.docChanged) return value;
            else return this.create(tr.state);
        },
    });

    function formattingHighlightDecorations(view: EditorView): DecorationSet {
        const result: Range<Decoration>[] = [];
        for (
            const cursor = view.state.field(formattingHighlights).iter();
            cursor.value !== null;
            cursor.next()
        ) {
            result.push({
                from: cursor.from,
                to: cursor.to,
                value: Decoration.mark({
                    inclusive: true,
                    tagName: cursor.value.value === "Bold" ? "b" : "i",
                }),
            });
        }
        return Decoration.set(result);
    }

    const formattingHighlightsPlugin = [
        formattingHighlights,
        ViewPlugin.fromClass(
            class {
                decorations: DecorationSet;

                constructor(view: EditorView) {
                    this.decorations = formattingHighlightDecorations(view);
                }

                update(update: ViewUpdate) {
                    if (
                        update.docChanged ||
                        update.viewportChanged ||
                        syntaxTree(update.startState) !=
                            syntaxTree(update.state)
                    )
                        this.decorations = formattingHighlightDecorations(
                            update.view,
                        );
                }
            },
            {
                decorations: (v) => v.decorations,
            },
        ),
    ];

    onMount(() => {
        view = new EditorView({
            doc: text,
            parent: dom,
            extensions: [
                EditorView.lineWrapping,
                EditorView.updateListener.of(
                    (update) => (text = update.state.doc.toString()),
                ),
                EditorView.theme(
                    {
                        ".cm-line": {
                            padding: 0,
                        },
                        ".cm-content": {
                            padding: 0,
                        },
                        ".cm-scroller": {
                            fontFamily: "sans-serif",
                        },
                        "&.cm-editor.cm-focused": {
                            outline: "none",
                        },
                    },
                    { dark: true },
                ),
                syntaxHighlighting(defaultHighlightStyle),
                xml(),
                keymap.of(defaultKeymap),
                treeView,
                tagDecorationsPlugin,
                formattingHighlightsPlugin,
            ],
        });
        view.focus();
        if (initialPosition !== undefined) {
            view.dispatch({
                selection: EditorSelection.single(initialPosition),
            });
        }

        return () => view!.destroy();
    });

    function insertFormatting(kind: "Bold" | "Italic") {
        if (view === undefined) return;
        const tag = kind === "Bold" ? "b" : "i";
        view.dispatch(
            view.state.changeByRange((range) => {
                const from = `<${tag}>`;
                const to = `</${tag}>`;
                const toChanges = ChangeSet.of(
                    { from: range.to, insert: to },
                    view!.state.doc.length,
                );
                const fromChanges = ChangeSet.of(
                    { from: range.from, insert: from },
                    view!.state.doc.length + to.length,
                );
                // view.state.field(formattingHighlights).between();
                return {
                    range: range.empty
                        ? range.map(toChanges).map(fromChanges, 0)
                        : range
                              .map(toChanges.compose(fromChanges))
                              .extend(
                                  range.from,
                                  range.to + from.length + to.length,
                              ),
                    changes: toChanges.compose(fromChanges),
                };
            }),
        );
    }
</script>

<div class="relative">
    <div class="absolute -top-8 flex gap-2 text-sm">
        <div
            class="dark:border-nord4/50 dark:bg-nord0 flex h-6 items-center gap-2 rounded border px-2 py-1"
        >
            <button
                class="font-bold"
                onmousedown={(e) => e.preventDefault()}
                onclick={() => {
                    insertFormatting("Bold");
                }}>B</button
            >
            <div class="dark:bg-nord4/50 h-full w-px"></div>
            <button
                class="hover:dark:bg-nord3 italic"
                onmousedown={(e) => e.preventDefault()}
                onclick={() => {
                    insertFormatting("Italic");
                }}>I</button
            >
        </div>
        <div
            class="dark:border-nord4/50 dark:bg-nord0 flex h-6 items-center rounded border px-0.5 py-0.5"
        >
            <button
                class:opacity-30={style !== "P"}
                class="h-full rounded px-1.5"
                onmousedown={(e) => e.preventDefault()}
                onclick={() => {
                    if (onSwitchStyle) onSwitchStyle("P");
                }}>P</button
            >
            {#each [1, 2, 3, 4, 5, 6] as const as level}
                <div
                    class="dark:bg-nord4/50 mx-0.5 h-[calc(100%-4px)] w-px"
                ></div>
                <button
                    class:opacity-30={style !== `H${level}`}
                    class="h-full rounded px-1.5 font-bold"
                    onmousedown={(e) => e.preventDefault()}
                    onclick={() => {
                        if (onSwitchStyle) onSwitchStyle(`H${level}`);
                    }}>H<sub class="relative top-0">{level}</sub></button
                >
            {/each}
        </div>
    </div>
    <div
        role="none"
        class:h1={style === "H1"}
        class:h2={style === "H2"}
        class:h3={style === "H3"}
        class:h4={style === "H4"}
        class:h5={style === "H5"}
        class:h6={style === "H6"}
        bind:this={dom}
    ></div>
</div>
