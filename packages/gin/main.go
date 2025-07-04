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

	router := gin.Default()
	router.Use(middleware.Logger())

	// Register /ping route
	router.GET("/ping", api.PingHandler)

	// Get port from config (env), default to 8080
	port := config.GetPort()
	log.Printf("Starting server on port %s", port)

	if err := router.Run(":" + port); err != nil {
		log.Fatalf("could not start server: %v", err)
	}
}
