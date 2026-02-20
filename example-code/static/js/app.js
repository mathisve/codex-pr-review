// Set min date on check-out when check-in changes
document.addEventListener('DOMContentLoaded', function () {
  var checkIn = document.getElementById('check_in');
  var checkOut = document.getElementById('check_out');
  if (checkIn && checkOut) {
    checkIn.addEventListener('change', function () {
      checkOut.min = checkIn.value;
      if (checkOut.value && checkOut.value < checkIn.value) {
        checkOut.value = checkIn.value;
      }
    });
  }
});
