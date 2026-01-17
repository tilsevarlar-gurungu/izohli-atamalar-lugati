document.addEventListener("DOMContentLoaded", function() {
    const abbrevations = document.querySelectorAll('abbr[data-en-ru]');

    // We store references to all adjust functions to update them on resize
    const allAdjusters = [];

    abbrevations.forEach(el => {
        // Create box if missing
        if (!el.querySelector('.tooltip-box')) {
            const hint = el.getAttribute('data-en-ru');
            const box = document.createElement('span');
            box.className = 'tooltip-box';
            box.textContent = hint;
            el.appendChild(box);
        }

        const box = el.querySelector('.tooltip-box');

        function adjustPosition() {
            // 1. Get Measurements
            const parentRect = el.getBoundingClientRect();
            const boxRect = box.getBoundingClientRect();
            const viewportWidth = document.documentElement.clientWidth;
            const padding = 15;

            // 2. Calculate "Ideal" Global Center
            let idealLeft = parentRect.left + (parentRect.width / 2) - (boxRect.width / 2);

            // 3. Clamp to Viewport Boundaries
            if (idealLeft < padding) {
                idealLeft = padding;
            }
            else if ((idealLeft + boxRect.width) > (viewportWidth - padding)) {
                idealLeft = viewportWidth - boxRect.width - padding;
            }

            // 4. Convert Global Coordinate back to Relative
            const finalRelativeLeft = idealLeft - parentRect.left;

            // 5. Apply
            box.style.left = `${finalRelativeLeft}px`;
        }

        // --- FIX #1: Run immediately! ---
        // This ensures the box snaps onto the screen instantly after creation,
        // preventing it from hanging off the edge and causing a scrollbar.
        adjustPosition();

        // Add to our list so we can update it if the user rotates their phone
        allAdjusters.push(adjustPosition);

        // Trigger on Hover/Touch
        el.addEventListener('mouseenter', adjustPosition);
        el.addEventListener('touchstart', adjustPosition, { passive: true });

        el.addEventListener('click', (e) => {
            e.stopPropagation();
            const isActive = el.classList.contains('active');
            document.querySelectorAll('abbr.active').forEach(a => a.classList.remove('active'));

            if (!isActive) {
                adjustPosition();
                el.classList.add('active');
            }
        });
    });

    // Close on global click
    document.addEventListener('click', () => {
        document.querySelectorAll('abbr.active').forEach(a => a.classList.remove('active'));
    });

    // --- FIX #2: Handle Screen Rotation ---
    // If the user rotates their phone, the screen width changes. 
    // We must recalculate all positions to keep them on screen.
    window.addEventListener('resize', () => {
        // Use requestAnimationFrame for performance
        window.requestAnimationFrame(() => {
            allAdjusters.forEach(adjust => adjust());
        });
    });
});
