package main

import (
	"log"

	"gin/api"
	"gin/config"
	"gin/middleware"

	"github.com/gin-gonic/gin"
	"github.com/joho/godotenv"
)

func main() {
	// Load environment variables from .env
	_ = godotenv.Load()

	// Initialize database connection
	config.InitDB()
	defer config.CloseDB()

	router := gin.Default()
	router.Use(middleware.Logger())

	// Health and status endpoints
	router.GET("/ping", api.PingHandler)
	router.GET("/health", api.HealthHandler)

	// User endpoints
	router.GET("/users", api.GetAllUsers)
	router.POST("/users", api.CreateUser)

	// Get port from config (env), default to 8080
	port := config.GetPort()
	log.Printf("Starting server on port %s", port)

	if err := router.Run(":" + port); err != nil {
		log.Fatalf("could not start server: %v", err)
	}
}
