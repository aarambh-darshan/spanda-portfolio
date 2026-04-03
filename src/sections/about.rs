use leptos::prelude::*;

#[component]
pub fn About() -> impl IntoView {
    view! {
        <section class="bg-code-bg text-white py-32 px-[5%]" style="clip-path: polygon(0 8%, 100% 0%, 100% 100%, 0% 100%);" id="about">
            <div class="reveal max-w-[1300px] mx-auto pt-16">
                <span class="font-mono text-accent text-sm tracking-widest uppercase">"// What is Spanda?"</span>
                <h2 class="font-instrument italic text-[4rem] lg:text-[6rem] text-white leading-none mt-6">
                    "The pulse of motion,"<br />
                    "written in Rust."
                </h2>
                <p class="font-syne text-white/60 text-lg lg:text-xl mt-8 max-w-[800px] leading-relaxed">
                    "Spanda (Sanskrit: स्पन्द — vibration, pulse) is a general-purpose animation library for Rust. Zero mandatory dependencies, "
                    <code class="font-code text-accent">"no_std"</code>"-ready, and designed to work anywhere: terminal UIs, web (WASM), game engines (Bevy), or native desktop apps."
                </p>

                <div class="flex flex-col md:flex-row justify-between items-center mt-20 border-t border-b border-accent/20 py-12 gap-12 md:gap-0">
                    <div class="flex flex-col items-center md:border-r border-accent/20 md:pr-12 w-full md:w-auto">
                        <div class="font-instrument italic text-[5rem] text-accent leading-none">"35+"</div>
                        <div class="font-mono text-xs uppercase text-white tracking-widest mt-2">"Easing Curves"</div>
                    </div>
                    <div class="flex flex-col items-center md:border-r border-accent/20 md:px-12 w-full md:w-auto">
                        <div class="font-instrument italic text-[5rem] text-accent leading-none">"0"</div>
                        <div class="font-mono text-xs uppercase text-white tracking-widest mt-2">"Unsafe Code"</div>
                    </div>
                    <div class="flex flex-col items-center md:border-r border-accent/20 md:px-12 w-full md:w-auto">
                        <div class="font-instrument italic text-[5rem] text-accent leading-none">"6"</div>
                        <div class="font-mono text-xs uppercase text-white tracking-widest mt-2">"Integration Targets"</div>
                    </div>
                    <div class="flex flex-col items-center md:pl-12 w-full md:w-auto">
                        <div class="font-instrument italic text-[5rem] text-accent leading-none">"v0.9.2"</div>
                        <div class="font-mono text-xs uppercase text-white tracking-widest mt-2">"Latest Release"</div>
                    </div>
                </div>

                <div class="flex flex-wrap gap-4 mt-16">
                    <span class="font-mono text-xs uppercase text-white border border-white/20 px-4 py-2 hover:bg-accent hover:text-black transition-colors">"🚀 GPU Compute"</span>
                    <span class="font-mono text-xs uppercase text-white border border-white/20 px-4 py-2 hover:bg-accent hover:text-black transition-colors">"📱 Gestures"</span>
                    <span class="font-mono text-xs uppercase text-white border border-white/20 px-4 py-2 hover:bg-accent hover:text-black transition-colors">"🔧 no_std"</span>
                    <span class="font-mono text-xs uppercase text-white border border-white/20 px-4 py-2 hover:bg-accent hover:text-black transition-colors">"🌐 WASM"</span>
                    <span class="font-mono text-xs uppercase text-white border border-white/20 px-4 py-2 hover:bg-accent hover:text-black transition-colors">"🎮 Bevy"</span>
                    <span class="font-mono text-xs uppercase text-white border border-white/20 px-4 py-2 hover:bg-accent hover:text-black transition-colors">"⚡ Tokio"</span>
                    <span class="font-mono text-xs uppercase text-white border border-white/20 px-4 py-2 hover:bg-accent hover:text-black transition-colors">"📦 Serde"</span>
                    <span class="font-mono text-xs uppercase text-white border border-white/20 px-4 py-2 hover:bg-accent hover:text-black transition-colors">"🎨 Palette"</span>
                </div>
            </div>
        </section>
    }
}
