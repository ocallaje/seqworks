(function($) {
    "use strict"; // Start of use strict
  
    // Close any open menu accordions when window is resized below 768px
    $(window).resize(function() {
      if ($(window).width() < 768) {
        $('.sidebar .collapse').collapse('hide');
      };
      
      // Toggle the side navigation when window is resized below 480px
      if ($(window).width() < 480 && !$(".sidebar").hasClass("toggled")) {
        $("body").addClass("sidebar-toggled");
        $(".sidebar").addClass("toggled");
        $('.sidebar .collapse').collapse('hide');
      };
    });
  
    // Prevent the content wrapper from scrolling when the fixed side navigation hovered over
    $('body.fixed-nav .sidebar').on('mousewheel DOMMouseScroll wheel', function(e) {
      if ($(window).width() > 768) {
        var e0 = e.originalEvent,
          delta = e0.wheelDelta || -e0.detail;
        this.scrollTop += (delta < 0 ? 1 : -1) * 30;
        e.preventDefault();
      }
    });
  
    // Scroll to top button appear
    $(document).on('scroll', function() {
      var scrollDistance = $(this).scrollTop();
      if (scrollDistance > 100) {
        $('.scroll-to-top').fadeIn();
      } else {
        $('.scroll-to-top').fadeOut();
      }
    });
  
    // Smooth scrolling using jQuery easing
    $(document).on('click', 'a.scroll-to-top', function(e) {
      var $anchor = $(this);
      $('html, body').stop().animate({
        scrollTop: ($($anchor.attr('href')).offset().top)
      }, 1000, 'easeInOutExpo');
      e.preventDefault();
    });
  
  })(jQuery); // End of use strict
  
  
  // Load sidebar and topbar when DOM content is loaded
  document.addEventListener('DOMContentLoaded', function() {
    loadTemplate('sidebar.html', 'sidebarContent');
    loadTemplate('topbar.html', 'topbarContent', setTodaysDate);
  });
  
  // Function to load a template
  function loadTemplate(templateUrl, targetId, callback) {
    var xhr = new XMLHttpRequest();
    xhr.onreadystatechange = function() {
      if (xhr.readyState === XMLHttpRequest.DONE) {
          if (xhr.status === 200) {
              document.getElementById(targetId).innerHTML = xhr.responseText;
              // Execute callback function if provided
              if (callback && typeof callback === 'function') {
                  callback();
              }
          } else {
              console.error('Failed to load template: ' + xhr.status);
          }
      }
    };
  xhr.open('GET', templateUrl, true);
  xhr.send();
}

  // Callback function to set today's date
  function setTodaysDate() {
    var today = new Date();
    var dd = String(today.getDate()).padStart(2, '0');
    var mm = String(today.getMonth() + 1).padStart(2, '0'); //January is 0!
    var yyyy = today.getFullYear();
    var hours = String(today.getHours()).padStart(2, '0');
    var minutes = String(today.getMinutes()).padStart(2, '0');

    var dateTime = dd + '/' + mm + '/' + yyyy + ' - ' + hours + ':' + minutes;
    document.getElementById('date').textContent = dateTime;
}
  // Event delegation to handle click on dynamically added elements
  $(document).on('click', "#sidebarToggle, #sidebarToggleTop", function(e) {
    $("body").toggleClass("sidebar-toggled");
    $(".sidebar").toggleClass("toggled");
    if ($(".sidebar").hasClass("toggled")) {
        $('.sidebar .collapse').collapse('hide');
    }
  });


  // Click Check-Buttons
function changeButtonClass(event, buttonId) {
  event.preventDefault();             // prevent auto scroll to top of page
    var button = document.getElementById(buttonId);
    if (button.classList.contains('btn-info')) {
        button.classList.remove('btn-info');
        button.classList.add('btn-light');
        button.setAttribute('data-clicked', 'false');
        button.querySelector('.icon').innerHTML = '<i class="fas fa-arrow-right"></i>'; // Revert the icon
        if (window.location.pathname.includes('pipe_bulk.html')) {
            if (button.id == "strandedness") {                  
            button.querySelector('.text').textContent = "Strandedness: Forward";
            } 
        } else if (window.location.pathname.includes('pipe_sc.html')) {
            if (button.id == "strandedness") {                  
            button.querySelector('.text').textContent = "Chemistry: Chromium V2";
            }
        }                
    } else if (button.classList.contains('btn-light')) {
        button.classList.remove('btn-light');
        button.classList.add('btn-info');
        button.setAttribute('data-clicked', 'true');
        button.querySelector('.icon').innerHTML = '<i class="fas fa-check"></i>'; // Change the icon
        if (window.location.pathname.includes('pipe_bulk.html')) {
            if (button.id == "strandedness") {                  
            button.querySelector('.text').textContent = "Strandedness: Reverse";
            } 
        } else if (window.location.pathname.includes('pipe_sc.html')) {
            if (button.id == "strandedness") {                  
            button.querySelector('.text').textContent = "Chemistry: Chromium V3";
            }
        }  
    }
}


// Dropdown field change
function updateDropdownValue(dropdownId, selectedValue) {
    var dropdownToggle = document.getElementById(dropdownId);
    dropdownToggle.textContent = selectedValue;
}

document.addEventListener("DOMContentLoaded", function() {
    var dropdowns = document.querySelectorAll('.dropdown-toggle');

    dropdowns.forEach(function(dropdownToggle) {
        var dropdownId = dropdownToggle.getAttribute('id');
        if (dropdownId) {
        var dropdownItems = document.querySelectorAll('#' + dropdownId + ' + .dropdown-menu .dropdown-item');

        dropdownItems.forEach(function(item) {
            item.addEventListener('click', function(event) {
                event.preventDefault();    // prevent auto scroll to top of page
                var selectedValue = item.textContent.trim();
                updateDropdownValue(dropdownId, selectedValue);
            });
        });
    }
    });
});
    