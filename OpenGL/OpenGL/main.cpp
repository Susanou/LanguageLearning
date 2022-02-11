#include <iostream>
#include <glad/glad.h>
#include<GLFW/glfw3.h>

int main()
{
	//Init GLFW
	glfwInit();

	//OPENGL version 3.3
	glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 3);
	glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 3);
	//Using the core profile (modern functions)
	glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);

	//Create the window object 800*800 not fullscreen
	GLFWwindow* window = glfwCreateWindow(800, 800, "screen test", NULL, NULL);
	//Check if window is created
	if (window == NULL) {
		std::cout << "Failed to create window" << ::std::endl;
		glfwTerminate();
		return -1;
	}
	//Introduce the window to the current context
	glfwMakeContextCurrent(window);

	//load GLAD to configure OPENGL
	gladLoadGL();

	//Specify viewport of the OpenGL window
	glViewport(0, 0, 800, 800);

	//Specify the color of the window
	glClearColor(0.07f, 0.13f, 0.17f, 1.0f);
	//Clean the back buffer and assign the new color to it
	glClear(GL_COLOR_BUFFER_BIT);
	//Swap the back and front buffers
	glfwSwapBuffers(window);

	float prev_time = float(glfwGetTime());
	float angle = 0.0f;

	//main while loop
	while (!glfwWindowShouldClose(window)) {

		float time = float(glfwGetTime());

		if (time - prev_time > 0.1f) {
			angle += 0.1f;
			prev_time = time;
		}

		glClearColor(float(sin(angle)), float(cos(angle)), float(tan(angle)), 1.0f);
		glClear(GL_COLOR_BUFFER_BIT);
		glfwSwapBuffers(window);
		glfwPollEvents();
	}

	//Cleanup so there isn't anything left hanging in memory
	glfwDestroyWindow(window);
	glfwTerminate();
	return 0;
}