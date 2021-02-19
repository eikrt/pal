package com.eikrt.app;
import java.io.File;
import java.io.FileNotFoundException;
import java.util.Scanner;
import java.util.ArrayList;
public class App 
{
    public static boolean isPalindrome(String s) {

	StringBuilder sb = new StringBuilder();
	sb.append(s);
	sb.reverse();
	
	return sb.toString().equals(s);
	
    }

    public static ArrayList<String> getWords() {

      ArrayList<String> words = new ArrayList<String>();
	try {
      File f = new File("../../../../../../data/data.txt");
      Scanner scanner = new Scanner(f);
      
      while (scanner.hasNextLine()) {
        String data = scanner.nextLine();
	words.add(data);
      }
      scanner.close();
    } catch (FileNotFoundException e) {
      System.out.println("An error occurred.");
      e.printStackTrace();
    }
return words;
    }

    public static void createPalindrome(ArrayList<String> words) {

	for (String word : words) {
	    for (String word2 : words) {
		for (String word3: words) {
		    if (isPalindrome(word+word2+word3)) {
			System.out.println(word+word2+word3);
		    }
		}
	    }
	}
    }
    

    public static void main( String[] args )
    {
	ArrayList<String> words = getWords();
	createPalindrome(words);
    }
}
