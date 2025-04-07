// Populate the sidebar
//
// This is a script, and not included directly in the page, to control the total size of the book.
// The TOC contains an entry for each page, so if each page includes a copy of the TOC,
// the total size of the page becomes O(n**2).
class MDBookSidebarScrollbox extends HTMLElement {
    constructor() {
        super();
    }
    connectedCallback() {
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded "><a href="index.html"><strong aria-hidden="true">1.</strong> Introduction</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="introduction/why.html"><strong aria-hidden="true">1.1.</strong> Why or Why Not</a></li><li class="chapter-item expanded "><a href="introduction/resources.html"><strong aria-hidden="true">1.2.</strong> Other Learning Resources</a></li></ol></li><li class="chapter-item expanded "><a href="getting-started/index.html"><strong aria-hidden="true">2.</strong> Getting Started</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="getting-started/setup.html"><strong aria-hidden="true">2.1.</strong> Getting Set Up</a></li><li class="chapter-item expanded "><a href="getting-started/environment.html"><strong aria-hidden="true">2.2.</strong> Environment</a></li><li class="chapter-item expanded "><a href="getting-started/hello-world.html"><strong aria-hidden="true">2.3.</strong> Hello World</a></li></ol></li><li class="chapter-item expanded "><a href="language-basics/index.html"><strong aria-hidden="true">3.</strong> Language Basics</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="language-basics/memory.html"><strong aria-hidden="true">3.1.</strong> Memory</a></li><li class="chapter-item expanded "><a href="language-basics/data-types.html"><strong aria-hidden="true">3.2.</strong> Data Types</a></li><li class="chapter-item expanded "><a href="language-basics/control-flow.html"><strong aria-hidden="true">3.3.</strong> Control Flow</a></li><li class="chapter-item expanded "><a href="language-basics/functions.html"><strong aria-hidden="true">3.4.</strong> Functions</a></li><li class="chapter-item expanded "><a href="language-basics/tests.html"><strong aria-hidden="true">3.5.</strong> Tests</a></li><li class="chapter-item expanded "><a href="language-basics/documentation.html"><strong aria-hidden="true">3.6.</strong> Documentation</a></li><li class="chapter-item expanded "><a href="language-basics/clippy-and-fmt.html"><strong aria-hidden="true">3.7.</strong> Clippy and Fmt</a></li><li class="chapter-item expanded "><a href="language-basics/impl.html"><strong aria-hidden="true">3.8.</strong> Giving types functionality</a></li><li class="chapter-item expanded "><a href="language-basics/traits.html"><strong aria-hidden="true">3.9.</strong> Traits Intro</a></li><li class="chapter-item expanded "><a href="language-basics/common-traits.html"><strong aria-hidden="true">3.10.</strong> Common Traits</a></li><li class="chapter-item expanded "><a href="language-basics/collections.html"><strong aria-hidden="true">3.11.</strong> Collections</a></li><li class="chapter-item expanded "><div><strong aria-hidden="true">3.12.</strong> Iterators</div></li><li class="chapter-item expanded "><div><strong aria-hidden="true">3.13.</strong> Attributes</div></li><li class="chapter-item expanded "><div><strong aria-hidden="true">3.14.</strong> Derive</div></li><li class="chapter-item expanded "><div><strong aria-hidden="true">3.15.</strong> Threads</div></li><li class="chapter-item expanded "><div><strong aria-hidden="true">3.16.</strong> Unsafe</div></li><li class="chapter-item expanded "><div><strong aria-hidden="true">3.17.</strong> Macros</div></li></ol></li><li class="chapter-item expanded "><div><strong aria-hidden="true">4.</strong> Common Patterns</div></li><li><ol class="section"><li class="chapter-item expanded "><div><strong aria-hidden="true">4.1.</strong> Derive</div></li><li class="chapter-item expanded "><div><strong aria-hidden="true">4.2.</strong> Monads</div></li><li class="chapter-item expanded "><div><strong aria-hidden="true">4.3.</strong> Interior Mutability</div></li><li class="chapter-item expanded "><div><strong aria-hidden="true">4.4.</strong> New Types</div></li><li class="chapter-item expanded "><div><strong aria-hidden="true">4.5.</strong> RAII / Drop and Finalise</div></li><li class="chapter-item expanded "><div><strong aria-hidden="true">4.6.</strong> Prefer Borrows</div></li><li class="chapter-item expanded "><div><strong aria-hidden="true">4.7.</strong> Type State</div></li><li class="chapter-item expanded "><div><strong aria-hidden="true">4.8.</strong> Builder</div></li></ol></li><li class="chapter-item expanded "><div><strong aria-hidden="true">5.</strong> Rust Ecosystem</div></li><li><ol class="section"><li class="chapter-item expanded "><div><strong aria-hidden="true">5.1.</strong> Project Structure</div></li><li class="chapter-item expanded "><div><strong aria-hidden="true">5.2.</strong> rustup</div></li><li class="chapter-item expanded "><div><strong aria-hidden="true">5.3.</strong> Error Handling with ThisError and Anyhow</div></li><li class="chapter-item expanded "><div><strong aria-hidden="true">5.4.</strong> log</div></li><li class="chapter-item expanded "><div><strong aria-hidden="true">5.5.</strong> mdbook</div></li><li class="chapter-item expanded "><div><strong aria-hidden="true">5.6.</strong> Itertools</div></li><li class="chapter-item expanded "><div><strong aria-hidden="true">5.7.</strong> Rayon</div></li><li class="chapter-item expanded "><div><strong aria-hidden="true">5.8.</strong> Clap</div></li><li class="chapter-item expanded "><div><strong aria-hidden="true">5.9.</strong> Async with Tokio and async_std</div></li><li class="chapter-item expanded "><div><strong aria-hidden="true">5.10.</strong> Serialisation with Serde</div></li><li class="chapter-item expanded "><div><strong aria-hidden="true">5.11.</strong> Parsing with Nom</div></li><li class="chapter-item expanded "><div><strong aria-hidden="true">5.12.</strong> Regex</div></li><li class="chapter-item expanded "><div><strong aria-hidden="true">5.13.</strong> reqwest</div></li><li class="chapter-item expanded "><div><strong aria-hidden="true">5.14.</strong> Crossbeam?</div></li><li class="chapter-item expanded "><div><strong aria-hidden="true">5.15.</strong> bitvec</div></li><li class="chapter-item expanded "><div><strong aria-hidden="true">5.16.</strong> Derive More</div></li></ol></li><li class="chapter-item expanded "><div><strong aria-hidden="true">6.</strong> Advanced Rust</div></li><li><ol class="section"><li class="chapter-item expanded "><div><strong aria-hidden="true">6.1.</strong> Foreign Function Interfaces</div></li><li class="chapter-item expanded "><div><strong aria-hidden="true">6.2.</strong> Proc Macro</div></li><li class="chapter-item expanded "><div><strong aria-hidden="true">6.3.</strong> Intro to Web Dev</div></li><li class="chapter-item expanded "><div><strong aria-hidden="true">6.4.</strong> Intro to Embedded</div></li><li class="chapter-item expanded "><div><strong aria-hidden="true">6.5.</strong> Intro to Game Dev</div></li></ol></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString().split("#")[0];
        if (current_page.endsWith("/")) {
            current_page += "index.html";
        }
        var links = Array.prototype.slice.call(this.querySelectorAll("a"));
        var l = links.length;
        for (var i = 0; i < l; ++i) {
            var link = links[i];
            var href = link.getAttribute("href");
            if (href && !href.startsWith("#") && !/^(?:[a-z+]+:)?\/\//.test(href)) {
                link.href = path_to_root + href;
            }
            // The "index" page is supposed to alias the first chapter in the book.
            if (link.href === current_page || (i === 0 && path_to_root === "" && current_page.endsWith("/index.html"))) {
                link.classList.add("active");
                var parent = link.parentElement;
                if (parent && parent.classList.contains("chapter-item")) {
                    parent.classList.add("expanded");
                }
                while (parent) {
                    if (parent.tagName === "LI" && parent.previousElementSibling) {
                        if (parent.previousElementSibling.classList.contains("chapter-item")) {
                            parent.previousElementSibling.classList.add("expanded");
                        }
                    }
                    parent = parent.parentElement;
                }
            }
        }
        // Track and set sidebar scroll position
        this.addEventListener('click', function(e) {
            if (e.target.tagName === 'A') {
                sessionStorage.setItem('sidebar-scroll', this.scrollTop);
            }
        }, { passive: true });
        var sidebarScrollTop = sessionStorage.getItem('sidebar-scroll');
        sessionStorage.removeItem('sidebar-scroll');
        if (sidebarScrollTop) {
            // preserve sidebar scroll position when navigating via links within sidebar
            this.scrollTop = sidebarScrollTop;
        } else {
            // scroll sidebar to current active section when navigating via "next/previous chapter" buttons
            var activeSection = document.querySelector('#sidebar .active');
            if (activeSection) {
                activeSection.scrollIntoView({ block: 'center' });
            }
        }
        // Toggle buttons
        var sidebarAnchorToggles = document.querySelectorAll('#sidebar a.toggle');
        function toggleSection(ev) {
            ev.currentTarget.parentElement.classList.toggle('expanded');
        }
        Array.from(sidebarAnchorToggles).forEach(function (el) {
            el.addEventListener('click', toggleSection);
        });
    }
}
window.customElements.define("mdbook-sidebar-scrollbox", MDBookSidebarScrollbox);
