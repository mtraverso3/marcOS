int current_line = 0, current_offset = 0;

void main() {
    char* video_memory = (char*) 0xb8000;
    *video_memory = 'X';

	char* str = "Hello World\n";

	clear_screen();
	print_32bit_str("\n\n", 0x0f);
	print_32bit_str(str, 0x4f);
}

void print_32bit_str(char* str, int color) {
	while(*str != '\0') {
		print_32bit_char(*str, color);
		str++;
	}
}

void print_32bit_char(char c, int color) {
	if (c == '\n') {
		current_offset = 0;
		current_line++;
		return;
	}

	char* video_memory = (char*) 0xb8000;
	if (current_offset >= 80) {
		current_offset = 0;
		current_line++;
	}
	int current_screen_location = (current_line * 80 + current_offset) * 2;

	*(video_memory + current_screen_location) = c;
	*(video_memory + current_screen_location + 1) = color;
	current_offset++;
}

void clear_screen(){
	char* video_memory = (char*) 0xb8000;

	for (int i = 0; i < 80 * 25; ++i) {
		*(video_memory + i * 2) = ' ';
		*(video_memory + i * 2 + 1) = 0x0f;
	}
}